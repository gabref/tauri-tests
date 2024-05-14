use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::body::{Bytes, Frame};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{body::Body, Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use tokio::net::TcpListener;

use crate::maestro::{Data, Operations, OutputData};

pub async fn run_server(
    addr: SocketAddr,
    server: HttpServer,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    // spawn background task to listen for maestro output data
    let maestro_output_r = server.maestro_receiver_output.clone();
    let is_processing = server.is_processing.clone();
    let last_operation = server.last_operation.clone();
    thread::spawn(move || listen_maestro_output(maestro_output_r, is_processing, last_operation));

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

fn listen_maestro_output(
    maestro_output_r: Arc<Mutex<crossbeam::channel::Receiver<OutputData>>>,
    is_processing: Arc<Mutex<bool>>,
    last_operation: Arc<Mutex<Option<OutputData>>>,
) {
    println!("Listening to maestro output");
    loop {
        let output_data = maestro_output_r.lock().unwrap().recv().unwrap();
        println!("Received something from maestro");
        println!("from maestro: {:#?}", output_data);
        let mut is_processing = is_processing.lock().unwrap();
        *is_processing = false;
        let mut last_operation = last_operation.lock().unwrap();
        *last_operation = Some(output_data);
    }
}

#[derive(Clone)]
pub struct HttpServer {
    maestro_sender: Arc<Mutex<crossbeam::channel::Sender<Operations>>>,
    maestro_sender_input: Arc<Mutex<crossbeam::channel::Sender<Data>>>,
    maestro_receiver_output: Arc<Mutex<crossbeam::channel::Receiver<OutputData>>>,
    last_operation: Arc<Mutex<Option<OutputData>>>,
    is_processing: Arc<Mutex<bool>>,
}

impl HttpServer {
    pub fn new(
        maestro_sender: Arc<Mutex<crossbeam::channel::Sender<Operations>>>,
        maestro_sender_input: Arc<Mutex<crossbeam::channel::Sender<Data>>>,
        maestro_receiver_output: Arc<Mutex<crossbeam::channel::Receiver<OutputData>>>,
    ) -> Self {
        Self {
            maestro_sender,
            maestro_sender_input,
            maestro_receiver_output,
            is_processing: Arc::new(Mutex::new(false)),
            last_operation: Arc::new(Mutex::new(None)),
        }
    }

    fn maestro_busy(&self) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
        let mut response = Response::new(Self::full("server is busyy"));
        *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
        Ok(response)
    }

    fn start_operation(&self, op: Operations) {
        let sender = self.maestro_sender.lock().unwrap();
        sender.send(op).unwrap();
        println!("Sended op to maestro")
    }

    fn parse_data(&self) {
        println!("Paring the data");
        let data = Data {
            value: 42,
            message: "My message".to_string(),
        };
        println!("Unlocking sender_input");
        let sender_input = self.maestro_sender_input.lock().unwrap();
        if let Err(e) = sender_input.send(data) {
            println!("Error happened sending data: {:#?}", e);
        }
        println!("Sended data to maestro");
    }

    fn operation_started(
        &self,
        mut is_processing: MutexGuard<bool>,
    ) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
        println!("Creating starting response");
        let mut response = Response::new(Self::full("started successfully"));
        *response.status_mut() = StatusCode::OK;
        println!("locking is processing");
        *is_processing = true;
        println!("sending repsonse");
        Ok(response)
    }

    /// This is our service handler. It receives a Request, routes on its
    /// path, and returns a Future of a Response.
    async fn handle_request(
        &self,
        req: Request<hyper::body::Incoming>,
    ) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
        match (req.method(), req.uri().path()) {
            (&Method::GET, "/wai") => {
                println!("got request to /wai");
                let is_processing = self.is_processing.lock().unwrap();
                println!("is_processing: {:#?}", *is_processing);
                if *is_processing == true {
                    return self.maestro_busy();
                }
                println!("Starting operation");
                self.start_operation(Operations::Waifu);
                // TODO: parse the input and start transaction
                self.parse_data();
                self.operation_started(is_processing)
            }
            (&Method::GET, "/start") => {
                println!("got request to /start");
                let is_processing = self.is_processing.lock().unwrap();
                println!("is_processing: {:#?}", *is_processing);
                if *is_processing == true {
                    return self.maestro_busy();
                }
                println!("Starting operation");
                self.start_operation(Operations::Pokemon);
                // TODO: parse the input and start transaction
                self.parse_data();
                self.operation_started(is_processing)
            }
            // Serve some instructions at /
            (&Method::GET, "/status") => {
                println!("got request to /status");
                let mut is_processing = self.is_processing.lock().unwrap();
                println!("is_processing: {:#?}", *is_processing);
                match &mut *is_processing {
                    false => {
                        let mut last_operation = self.last_operation.lock().unwrap();
                        match &mut *last_operation {
                            Some(op) => {
                                let mut response = Response::new(Self::full(format!("{:#?}", op)));
                                *response.status_mut() = StatusCode::OK;
                                Ok(response)
                            }
                            None => {
                                let mut response = Response::new(Self::full("no op until now"));
                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                Ok(response)
                            }
                        }
                    }
                    true => self.maestro_busy(),
                }
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
