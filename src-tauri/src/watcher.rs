use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::maestro::Operations;

// Define a struct to hold the file watcher state
pub struct FileWatcher {
    directory: String,
    maestro_sender: Arc<Mutex<crossbeam::channel::Sender<Operations>>>,
}

impl FileWatcher {
    pub fn new(
        directory: String,
        maestro_sender: Arc<Mutex<crossbeam::channel::Sender<Operations>>>,
    ) -> Self {
        Self {
            directory,
            maestro_sender,
        }
    }

    pub fn start_watching(&self) {
        let path = Path::new(&self.directory);
        if !path.exists() {
            panic!("Invalid directory path! does not exists");
        }
        if !path.is_dir() {
            panic!("path is not a dir");
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
