i have this code for the server.rs

use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::body::{Bytes, Frame};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{body::Body, Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

use crate::maestro::Operations;

pub async fn run_server(
    addr: SocketAddr,
    server: HttpServer,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let server_clone = server.clone();
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(
                    io,
                    service_fn(move |req| {
                        let server_clone = server_clone.clone();
                        async move { server_clone.handle_request(req).await }
                    }),
                )
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

pub struct Data {
    value: i32,
    message: String,
}

#[derive(Clone)]
pub struct HttpServer {
    maestro_sender: Arc<Mutex<crossbeam::channel::Sender<Operations>>>,
    maestro_sender_input: Arc<Mutex<crossbeam::channel::Sender<Data>>>,
    is_processing: Arc<Mutex<bool>>,
}

impl HttpServer {
    pub fn new(
        maestro_sender: Arc<Mutex<crossbeam::channel::Sender<Operations>>>,
        maestro_sender_input: Arc<Mutex<crossbeam::channel::Sender<Data>>>,
    ) -> Self {
        Self {
            maestro_sender,
            maestro_sender_input,
            is_processing: Arc::new(Mutex::new(false)),
        }
    }

    fn maestro_busy(&self) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
        let mut response = Response::new(Self::empty());
        *response.status_mut() = StatusCode::PROCESSING;
        Ok(response)
    }

    fn start_operation(&self, op: Operations) {
        let sender = self.maestro_sender.lock().unwrap();
        sender.send(op).unwrap();
    }

    fn operation_started(&self) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
        let mut response = Response::new(Self::full("started successfully"));
        *response.status_mut() = StatusCode::OK;
        let mut is_processing = self.is_processing.lock().unwrap();
        *is_processing = true;
        Ok(response)
    }

    fn operation_finished(&self) {
        let mut is_processing = self.is_processing.lock().unwrap();
        *is_processing = false;
    }

    /// This is our service handler. It receives a Request, routes on its
    /// path, and returns a Future of a Response.
    async fn handle_request(
        &self,
        req: Request<hyper::body::Incoming>,
    ) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
        match (req.method(), req.uri().path()) {
            (&Method::GET, "/start") => {
                let is_processing = self.is_processing.lock().unwrap();
                if *is_processing {
                    return self.maestro_busy()
                }
                self.start_operation(Operations::Pokemon);
                // TODO: parse the input and start transaction
                let data = Data { value: 37, message: "My message".to_string() };
                let sender_input = self.maestro_sender_input.lock().unwrap();
                sender_input.send(data);
                self.operation_started()
            }
            // Serve some instructions at /
            (&Method::GET, "/") => Ok(Response::new(Self::full(
                "Try POSTing data to /echo such as: `curl localhost:3000/echo -XPOST -d \"hello world\"`",
            ))),
            // Simply echo the body back to the client.
            (&Method::POST, "/echo") => Ok(Response::new(req.into_body().boxed())),
            // Convert to uppercase before sending back to client using a stream.
            (&Method::POST, "/echo/uppercase") => {
                let frame_stream = req.into_body().map_frame(|frame| {
                    let frame = if let Ok(data) = frame.into_data() {
                        data.iter()
                            .map(|byte| byte.to_ascii_uppercase())
                            .collect::<Bytes>()
                    } else {
                        Bytes::new()
                    };
                    Frame::data(frame)
                });
                Ok(Response::new(frame_stream.boxed()))
            }

            // Reverse the entire body before sending back to the client.
            //
            // Since we don't know the end yet, we can't simply stream
            // the chunks as they arrive as we did with the above uppercase endpoint.
            // So here we do `.await` on the future, waiting on concatenating the full body,
            // then afterwards the content can be reversed. Only then can we return a `Response`.
            (&Method::POST, "/echo/reversed") => {
                // To protect our server, reject requests with bodies larger than
                // 64kbs of data.
                let max = req.body().size_hint().upper().unwrap_or(u64::MAX);
                if max > 1024 * 64 {
                    let mut resp = Response::new(Self::full("Body too big"));
                    *resp.status_mut() = hyper::StatusCode::PAYLOAD_TOO_LARGE;
                    return Ok(resp);
                }
                let whole_body = req.collect().await?.to_bytes();
                let reversed_body = whole_body.iter().rev().cloned().collect::<Vec<u8>>();
                Ok(Response::new(Self::full(reversed_body)))

            }

            // Return the 404 Not Found for other routes.
            _ => {
                let mut not_found = Response::new(Self::empty());
                *not_found.status_mut() = StatusCode::NOT_FOUND;
                Ok(not_found)
            }
        }
    }

    fn empty() -> BoxBody<Bytes, hyper::Error> {
        Empty::<Bytes>::new()
            .map_err(|never| match never {})
            .boxed()
    }

    fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
        Full::new(chunk.into())
            .map_err(|never| match never {})
            .boxed()
    }
}

and for the maestro.rs

use std::sync::{Arc, Mutex};

use tauri::AppHandle;
use tauri_plugin_positioner::{Position, WindowExt};
use tauri::Manager;

use crate::{server::{run_server, HttpServer}, watcher::FileWatcher};

pub enum Operations {
    Pix,
    Tef,
    Pokemon,
}

fn open_window(app_handle: AppHandle) {
    let window = app_handle.get_window("main").unwrap();
    window.show().unwrap();
    window.move_window(Position::Center).unwrap();
}

pub fn start_maestro(app_handle: AppHandle) {
    let (maestro_sender, maestro_receiver) = crossbeam::channel::bounded(1);
    let (maestro_sender_input, maestro_receiver_input) = crossbeam::channel::bounded(1);
    let maestro_sender = Arc::new(Mutex::new(maestro_sender));
    let maestro_sender_input = Arc::new(Mutex::new(maestro_sender_input));

    let file_watcher = FileWatcher::new("/home/gabre/work".to_string(), maestro_sender.clone());
    file_watcher.start_watching();

    let http_server = HttpServer::new(maestro_sender.clone(), maestro_sender_input.clone());
    let addr = ([127, 0, 0, 1], 8080).into();
    tokio::runtime::Runtime::new().unwrap().block_on(run_server(addr, http_server));

    println!("everythin started");
}

now, when the server send to maestro the data, maestro will run the transaction flow. for now, we can just sleep the thread maestro for testing, then, when the maestro has finished processing, the maestro thread will send back to the server (by channels, or by callback, or any other thing suitable) the output information, that the server will use, like the info of the last transaction, and the is_processing field from the server will be false
