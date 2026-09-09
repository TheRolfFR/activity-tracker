// Tray icon module inspired by https://github.com/andrewromanyk/EyeCatcher/blob/5106bfcaf7a90024832bb1e76a45acc175951fec/src/tray.rs

use std::time::Duration;

use tray_icon::{Icon, MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent, menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem}};
use iced::{task::{Never, Sipper, sipper}, Subscription};

use crate::state::MainMessage;

pub fn tray_icon<S>(name: S, icon_raw: &[u8]) -> TrayIcon
where S: AsRef<str> {
    let icon_image = image
        ::load_from_memory(icon_raw)
        .expect("No tray icon image found.")
        .to_rgba8();

    let (width, height) = icon_image.dimensions();

    #[cfg(target_os = "linux")]
    gtk::init().unwrap();

    let tray_menu = Menu::with_items(&[
        &MenuItem::with_id("quit", "Quit", true, None),
        &MenuItem::with_id("week_data", "Week data", true, None),
        &PredefinedMenuItem::separator(),
        &MenuItem::with_id("hide", "Hide", true, None),
    ]).expect("Failed to create menu items");
    TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_menu_on_left_click(false)
        .with_menu_on_right_click(true)
        .with_tooltip(name)
        .with_icon(Icon::from_rgba(icon_image.into_raw(), width, height).expect("Could not instantiate icon image for tray."))
        .build()
        .expect("Could not start tray icon.")
}

fn tray_stream() -> impl Sipper<Never, MainMessage> {
    sipper(async |mut output| {
        let menu_receiver = MenuEvent::receiver();
        let icon_receiver = TrayIconEvent::receiver();

        let mut interval = tokio::time::interval(Duration::from_millis(100));

        loop {
            interval.tick().await;
            if let Ok(event) = menu_receiver.try_recv() {
                output.send(MainMessage::TrayEvent(event.id().0.clone())).await;
            }
            while let Ok(event) = icon_receiver.try_recv() {
                match event {
                    TrayIconEvent::Click { id: _, position: _, rect: _, button: MouseButton::Left, button_state: MouseButtonState::Up  } => {
                        output.send(MainMessage::TrayIconClick).await;
                    },
                    _ => {}
                }
            }
        }
    })
}

pub fn tray_subscription() -> Subscription<MainMessage> {
    Subscription::run(tray_stream)
}
