#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::vec;
use std::thread;
use std::sync::mpsc::channel;

use payload::Payload;
use tauri::{SystemTray, SystemTrayEvent};
use rdev::Event;
use tauri::Manager;
use tauri_plugin_window_state;


mod threads;
use threads::*;

mod day_record;
mod day_record_file;

mod data;

mod payload;

mod week_stats;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let (evt_schan, evt_rchan) = channel::<Event>();
    let (payload_schan, payload_rchan) = channel::<()>();

    event_thread(evt_schan);

    thread::spawn(move || -> () {
        day_thread(evt_rchan, payload_schan);
    });

    let system_tray = SystemTray::new();
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                ..
            } => {
                let main_window = app.get_window("main").unwrap();
                if main_window.is_visible().unwrap() {
                    main_window.hide().ok();
                } else {
                    main_window.show().ok();
                }
            }
            _ => {}
        })
        .system_tray(system_tray)
        .invoke_handler(tauri::generate_handler![greet])
        .setup(move |app| {

            let main_window = app.get_window("main").unwrap();
            
            thread::spawn(move || {
                loop {
                    match payload_rchan.recv() {
                        Ok(_) => {
                            let payload = Payload::new();
                            main_window.emit("activity", payload).ok();
                        },
                        Err(_) => {
                            // nope do nothing
                        },
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
