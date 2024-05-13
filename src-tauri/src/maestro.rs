use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crossbeam::channel::Sender;
use tauri::{AppHandle, PhysicalPosition};
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
    pub value: u32,
    pub message: String,
}

#[derive(Debug)]
pub struct OutputData {
    pub status_code: u32,
    pub status_message: String,
}

fn open_window(app_handle: &AppHandle) {
    let window = app_handle.get_window("main").unwrap();
    window.show().unwrap();
    window.move_window(Position::Center).unwrap();
}

pub fn start_maestro(app_handle: AppHandle) {
    thread::spawn(|| start_maestro_thread(app_handle));
}

fn start_maestro_thread(app_handle: AppHandle) {
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
            Ok(operation) => {
                println!("got operation, stop all threads, and do action");
                match operation {
                    Operations::Pokemon => match maestro_receiver_input.recv() {
                        Ok(data) => {
                            do_pok_op(data, maestro_output_s.clone(), &app_handle);
                        }
                        Err(_) => {
                            println!("Error occurred in maestro receiver input");
                            break;
                        }
                    },
                    _ => {
                        println!("Other actions");
                    }
                }
            }
            Err(_) => {
                println!("Error occurred in maestro receiver");
                break;
            }
        }
    }
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

fn do_pok_op(data: Data, sender: Sender<OutputData>, app_handle: &AppHandle) {
    open_window(app_handle);

    app_handle.emit_all("push", Payload { message: "this is the message".into() });

    println!("Will start a Pokemon actions");
    thread::sleep(Duration::from_secs(1));
    println!("processing 1...");
    thread::sleep(Duration::from_secs(1));
    println!("processing 2...");
    thread::sleep(Duration::from_secs(1));
    println!("processing 3...");

    let output = OutputData {
        status_code: data.value,
        status_message: data.message,
    };
    sender.send(output).unwrap();
}
