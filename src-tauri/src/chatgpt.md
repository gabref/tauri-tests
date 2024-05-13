I have this code:

main.rs
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod server;
mod maestro;
mod watcher;

use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
use tauri_plugin_positioner::{Position, WindowExt};
#[allow(unused_imports)]
use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState};

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit").accelerator("ctrl+alt+q");
    let system_tray_menu = SystemTrayMenu::new().add_item(quit);

    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .system_tray(SystemTray::new().with_menu(system_tray_menu))
        .setup(|app| {
            app.listen_global("quit", |_| {
                std::process::exit(0);
            });

            let _window = app.get_window("main").unwrap();

            #[cfg(target_os = "macos")]
            apply_vibrancy(&_window, NSVisualEffectMaterial::HudWindow, None, None)
                .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            #[cfg(target_os = "windows")]
            apply_blur(&_window, Some((18, 18, 18, 125)))
                .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

            println!("passing handle");
            maestro::start_maestro(app.handle());
            println!("handle passed");
            Ok(())
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .on_system_tray_event(|app, event| {
            tauri_plugin_positioner::on_tray_event(app, &event);
            match event {
                SystemTrayEvent::LeftClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    let window = app.get_window("main").unwrap();
                    let _ = window.move_window(Position::TrayCenter);
                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                }
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                },
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

maestro.rs
use std::sync::{Arc, Mutex};

use tauri::AppHandle;
use tauri_plugin_positioner::{Position, WindowExt};
use tauri::Manager;

use crate::{server::HttpServer, watcher::FileWatcher};

fn open_window(app_handle: AppHandle) {
    let window = app_handle.get_window("main").unwrap();
    window.show().unwrap();
    window.move_window(Position::Center).unwrap();
}

pub fn start_maestro(app_handle: AppHandle) {
    let (maestro_sender, maestro_receiver) = crossbeam::channel::bounded(100);
    let maestro_sender = Arc::new(Mutex::new(maestro_sender));

    let file_watcher = FileWatcher::new("/home/gabre/work".to_string(), maestro_sender.clone());
    file_watcher.start_watching();

    let http_server = HttpServer::new(maestro_sender.clone());
    let addr = ([127, 0, 0, 1], 8080).into();
    tokio::runtime::Runtime::new().unwrap().block_on(http_server.run_server(addr));

    println!("everythin started");
}

server.rs
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::body::{Bytes, Frame};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{body::Body, Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

pub struct HttpServer {
    maestro_sender: Arc<Mutex<crossbeam::channel::Sender<String>>>,
}

impl HttpServer {
    pub fn new(maestro_sender: Arc<Mutex<crossbeam::channel::Sender<String>>>) -> Self {
        Self { maestro_sender }
    }

    /// This is our service handler. It receives a Request, routes on its
    /// path, and returns a Future of a Response.
    async fn handle_request(
        req: Request<hyper::body::Incoming>,
    ) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
        match (req.method(), req.uri().path()) {
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

    pub async fn run_server(
        &self,
        addr: SocketAddr,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

        let listener = TcpListener::bind(addr).await?;
        println!("Listening on http://{}", addr);

        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);
            tokio::task::spawn(async move {
                if let Err(err) = http1::Builder::new()
                    .serve_connection(io, service_fn(Self::handle_request))
                    .await
                {
                    println!("Error serving connection: {:?}", err);
                }
            });
        }
    }
}

watcher.rs
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Define a struct to hold the file watcher state
pub struct FileWatcher {
    directory: String,
    maestro_sender: Arc<Mutex<crossbeam::channel::Sender<String>>>,
}

impl FileWatcher {
    pub fn new(
        directory: String,
        maestro_sender: Arc<Mutex<crossbeam::channel::Sender<String>>>,
    ) -> Self {
        Self {
            directory,
            maestro_sender,
        }
    }

    pub fn start_watching(&self) {
        let path = Path::new(&self.directory);
        if !path.exists() || !path.is_dir() {
            panic!("Invalid directory path!");
        }

        thread::spawn(move || {
            loop {
                // match fs::read_dir(&path) {
                //     Ok(entries) => {
                //         for entry in entries {
                //             if let Ok(entry) = entry {
                //                 let file_path = entry.path();
                //                 if file_path.is_file() {
                //                     if let Ok(mut file) = fs::File::open(&file_path) {
                //                         let mut contents = String::new();
                //                         if let Ok(_) = file.read_to_string(&mut contents) {
                //                             // Send contents to maestro thread
                //                             let sender = self.maestro_sender.lock().unwrap();
                //
                //                             sender.send(contents).unwrap();
                //                             // Delete the file after reading
                //                             if let Err(err) = fs::remove_file(&file_path) {
                //                                 eprintln!("Error deleting file: {}", err);
                //                             }
                //                         }
                //                     }
                //                 }
                //             }
                //         }
                //     }
                //     Err(err) => {
                //         eprintln!("Error reading directory: {}", err);
                //     }
                // }
                thread::sleep(Duration::from_secs(5)); // Check every 5 seconds
                println!("reading dir, waiting for file")
            }
        });
    }
}

so, now, i want to implement the following: add a endoint in the server '/transaction-pix' that will immediately tell the maestro thread that input has come and a transaction will begin. so the maestro thread has to tell to server and watcher to stop receiving input. in the case of the server, the server will need to respond with a status code and message showing that it is busy right now. then the /transaction-pix after notifing the maestro thread will continue, parse the input receive, and then pass the parsed values to the maestro thread, which will then start the transaction flow. when the transaction has ended, the maestro thread notifies the server and watcher so that they can start receiving input again.
remember, i want to follow the method where I use structs and methods
