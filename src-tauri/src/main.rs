// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use tauri::{AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
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
            setup_server(app.handle());
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

fn handle_client(mut stream: TcpStream, app_handle: AppHandle) {
    println!("received request");
    // Read the incoming HTTP request
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // Assuming the request is a simple GET request for "/"
    let response = "HTTP/1.1 200 OK\r\n\r\nHello, World!";

    // Send the response back to the client
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    let window = app_handle.get_window("main").unwrap();
    window.show().unwrap();
    window.move_window(Position::Center).unwrap();
    println!("client handled");
}

fn start_server(app_handle: AppHandle) {
    let listener = TcpListener::bind("127.0.0.1:4242").unwrap();
    println!("Server listening on port 8080...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let app_handle_clone = app_handle.clone();
                thread::spawn(move || {
                    handle_client(stream, app_handle_clone);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}

fn setup_server(app_handle: AppHandle) {
    thread::spawn(|| {
        start_server(app_handle);
    });
}
