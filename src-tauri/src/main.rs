// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rodio::source::{SineWave, Source};
use rodio::{Decoder, OutputStream, Sink};
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

use std::sync::Mutex;
use tauri::State;

struct Storage {
    store: Mutex<Sink>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn list_stuff() -> [i32; 5] {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    return numbers;
}

#[tauri::command]
fn get_files() -> Vec<String> {
    let paths = fs::read_dir("./songs").unwrap();

    let mut array: Vec<String> = Vec::new();
    for path in paths {
        let dir = path.unwrap();
        if (dir.path().extension().unwrap() != "mp3") {continue;}
        array.push(dir.path().display().to_string());
    }

    return array;
}

#[tauri::command(async)]
fn play_song(songname: &str, storage: State<Storage>) {
    let sink = storage.store.lock().unwrap();
    sink.stop();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(songname).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    sink.append(source);
    sink.set_volume(0.25);
    // sink.sleep_until_end();
}

fn main() {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink: Sink = Sink::try_new(&stream_handle).unwrap();
    tauri::Builder::default()
    .manage(Storage { store: sink.into() })
    .invoke_handler(tauri::generate_handler![
            greet, list_stuff, get_files, play_song
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
