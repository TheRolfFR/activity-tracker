#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::vec;
use std::thread;
use std::sync::mpsc::channel;

use tauri::Manager;

pub mod data;

mod event_thread;
use event_thread::event_thread;

mod activity_thread;
use activity_thread::activity_thread;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let (act_schan, act_rchan) = channel();
    let (evt_schan, evt_rchan) = channel();

    event_thread(evt_schan);

    activity_thread(evt_rchan, act_schan);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(move |app| {

            let main_window = app.get_window("main").unwrap();
            main_window.set_title("Activity tracker").unwrap();
            main_window.set_always_on_top(true).unwrap();

            thread::spawn(move || {
                loop {
                    if let Ok(act) = act_rchan.recv() {
                        main_window.emit("activity", act).ok();
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
