#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::mpsc::SyncSender;
use std::sync::mpsc::sync_channel;
use std::vec;
use std::thread;
use std::sync::mpsc::channel;

use log::error;
use log::{LevelFilter, info};
use simple_logging;
use tauri::{SystemTray, SystemTrayEvent};
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};
use tauri::Manager;
use tauri_plugin_window_state;
use tauri_plugin_window_state::{AppHandleExt, StateFlags};

mod throttle;

mod day_record;
mod day_record_file;
mod records;

mod threads;
use threads::*;
use week_data::Payload;

mod data;
mod week_data;
mod week_stats;

#[allow(dead_code)]
#[tauri::command]
fn open_week_window(app_handle: tauri::AppHandle) {
    app_handle.get_window("week_data")
    .and_then(|w| w.show().ok());
}

#[allow(dead_code)]
#[tauri::command]
fn adjust(val: i32, state: tauri::State<SyncSender<i32>>) {
    if let Err(err) = state.send(val) {
        error!("Failed to send adjustment: {err}");
    }
}

#[allow(dead_code)]
#[tauri::command]
fn open_menu_handle(window: tauri::Window) {
    println!("Coucou");
    if let Err(err) = window.menu_handle().show() {
        error!("Failed to open menu {err}");
    }
}

fn main() {
    //* Log
    simple_logging::log_to_file(
        std::env::temp_dir().join("activity-tracker.log"),
        LevelFilter::Trace
    ).unwrap();

    info!("Starting activity tracker...");

    let (evt_schan, evt_rchan) = channel::<DataMessage>();
    let send_adj = evt_schan.clone();
    let (payload_schan, payload_rchan) = channel::<Payload>();
    let (adj_schan, adj_rchan) = sync_channel::<i32>(1);

    thread::spawn(move || -> () {
        adjustment_thread(adj_rchan, send_adj);
    });

    event_thread(evt_schan);

    thread::spawn(move || -> () {
        data_thread(evt_rchan, payload_schan);
    });

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let week = CustomMenuItem::new("week_data".to_string(), "Week data");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
    .add_item(quit)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(hide)
    .add_item(week);
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .manage(adj_schan)
        .invoke_handler(tauri::generate_handler![adjust, open_menu_handle, open_week_window])
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                ..
            } => {
                let opt_main_window = app.get_window("main");
                
                if let Some(main_window) = opt_main_window {
                    if main_window.is_visible().unwrap() {
                        main_window.hide().ok();
                    } else {
                        main_window.show().ok();
                    }
                    app.save_window_state(StateFlags::VISIBLE|StateFlags::POSITION|StateFlags::SIZE).ok();
                }
            },
            SystemTrayEvent::MenuItemClick { id, .. } => {
              // get a handle to the clicked menu item
              // note that `tray_handle` can be called anywhere,
              // just get a `AppHandle` instance with `app.handle()` on the setup hook
              // and move it to another function or thread
              match id.as_str() {
                "quit" => {
                  app.save_window_state(StateFlags::all())
                  .and_then(|_| -> Result<(), tauri_plugin_window_state::Error> {
                    log::info!("Successully saved window state");
                    Ok(())
                  })
                  .or_else(|_| -> Result<(), ()> {
                    log::error!("Failed to save window state");
                    Ok(())
                  }).ok();
                  app.exit(0);
                }
                "week_data" => {
                    app.get_window("week_data")
                        .and_then(|w| w.show().ok());
                }
                "hide" => {
                  for w in app.windows().values() {
                    w.hide().unwrap();
                  }
                  app.save_window_state(StateFlags::VISIBLE|StateFlags::POSITION|StateFlags::SIZE).ok();
                }
                _ => {}
              }
            },
            _ => {}
        })
        .setup(move |app| {

            let main_window = app.get_window("main").unwrap();
            let ver = app.package_info().version.to_string();
            
            thread::spawn(move || {
                loop {
                    match payload_rchan.recv() {
                        Ok(mut payload) => {
                            payload.version = format!("{}-{}-{}", ver, env!("GIT_HASH"), env!("GIT_DATE"));
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
