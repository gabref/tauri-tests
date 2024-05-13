use std::sync::{Arc, Mutex};

use tauri::AppHandle;
use tauri_plugin_positioner::{Position, WindowExt};
use tauri::Manager;

use crate::{server::HttpServer, watcher::FileWatcher};

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
    let (maestro_sender, maestro_receiver) = crossbeam::channel::bounded(100);
    let maestro_sender = Arc::new(Mutex::new(maestro_sender));

    let file_watcher = FileWatcher::new("/home/gabre/work".to_string(), maestro_sender.clone());
    file_watcher.start_watching();

    let http_server = HttpServer::new(maestro_sender.clone());
    let addr = ([127, 0, 0, 1], 8080).into();
    tokio::runtime::Runtime::new().unwrap().block_on(http_server.run_server(addr));

    println!("everythin started");
}
