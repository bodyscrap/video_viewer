mod video_reader;
use tauri::{State, ipc::Channel};
use std::sync::{Arc, Mutex};
use gst;
use video_reader::{VideoReader, ImageBuffer};
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn init(video_path: &str, on_event:Channel<ImageBuffer>,  state: State<'_, Arc<Mutex<VideoReader>>>) -> bool {
    println!("call init command");
    let mut state = state.lock().unwrap();
    !state.init(video_path, on_event).is_err()
}

#[tauri::command]
fn play(state: State<'_, Arc<Mutex<VideoReader>>>) -> bool {
    println!("call play command");
    let state = state.lock().unwrap();
    !state.play().is_err()
}

#[tauri::command]
fn stop(state: State<'_, Arc<Mutex<VideoReader>>>) -> bool {
    println!("call stop command");
    let mut state = state.lock().unwrap();
    !state.stop().is_err()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    gst::init().unwrap();
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(Arc::new(Mutex::new(VideoReader::default()))) // Stateを管理
        .invoke_handler(tauri::generate_handler![init, play, stop])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
