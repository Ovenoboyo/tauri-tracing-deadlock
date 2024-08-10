// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    thread::{self, sleep},
    time::Duration,
};

use tauri::{AppHandle, Manager};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command(async)]
fn greet() {
    println!("calling greet");
    // loop {
    //     println!("Sending event");
    //     app.emit("hello", "bye").unwrap();
    //     println!("sent event");
    // }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let app_handle = app.handle().clone();
            let app_handle1 = app.handle().clone();
            thread::spawn(move || loop {
                app_handle.emit("hello", "bye").unwrap();
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
