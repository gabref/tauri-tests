use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use tauri::AppHandle;
use tauri::Manager;
use tauri_plugin_positioner::{Position, WindowExt};

use crate::{
    server::{run_server, HttpServer},
    watcher::FileWatcher,
};

pub enum Operations {
    Pix,
    Tef,
    Pokemon,
}

pub struct Data {
    pub value: i32,
    pub message: String,
}

#[derive(Debug)]
pub struct OutputData {
    pub status_code: u32,
    pub status_message: String,
}

fn open_window(app_handle: AppHandle) {
    let window = app_handle.get_window("main").unwrap();
    window.show().unwrap();
    window.move_window(Position::Center).unwrap();
}

pub fn start_maestro(app_handle: AppHandle) {
    let (maestro_sender, maestro_receiver) = crossbeam::channel::bounded(1);
    let (maestro_sender_input, maestro_receiver_input) = crossbeam::channel::bounded(1);
    let (maestro_output_s, maestro_output_r) = crossbeam::channel::bounded(1);

    let maestro_sender = Arc::new(Mutex::new(maestro_sender));
    let maestro_sender_input = Arc::new(Mutex::new(maestro_sender_input));
    let maestro_output_r = Arc::new(Mutex::new(maestro_output_r));

    let file_watcher = FileWatcher::new(
        "C:\\Users\\codec\\Documents\\embed".to_string(),
        maestro_sender.clone(),
    );
    file_watcher.start_watching();

    // spawn the server in a separate asynchronous task
    thread::spawn(move || {
        let http_server = HttpServer::new(
            maestro_sender.clone(),
            maestro_sender_input.clone(),
            maestro_output_r.clone(),
        );
        let addr = ([127, 0, 0, 1], 8080).into();

        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(run_server(addr, http_server));
    });

    println!("everythin started");

    loop {
        match maestro_receiver.recv() {
            Ok(operation) => match operation {
                Operations::Pokemon => {
                    println!("Will start a Pokemon actions");
                    thread::sleep(Duration::from_secs(1));
                    println!("processing 1...");
                    thread::sleep(Duration::from_secs(1));
                    println!("processing 2...");
                    thread::sleep(Duration::from_secs(1));
                    println!("processing 3...");

                    let output = OutputData {
                        status_code: 42,
                        status_message: "Pokemon success".to_string(),
                    };
                    maestro_output_s.send(output).unwrap();
                }
                _ => {
                    println!("Other actions");
                }
            },
            Err(_) => {
                println!("Error occurred in maestro receiver");
                break;
            }
        }
    }
}
