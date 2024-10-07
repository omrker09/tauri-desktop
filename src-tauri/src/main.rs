#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{ Window};

// Prevents additional console window on Windows in release, DO NOT REMOVE!!


#[tauri::command]
async fn close_splashscreen(window: Window) {
  // Close splashscreen
  window.get_window("splashscreen").expect("no window labeled 'splashscreen' found").close().unwrap();
  // Show main window
  window.get_window("main").expect("no window labeled 'main' found").show().unwrap();
}

fn main() {
    secure_chat_lib::run()
    
}
