use chrono::{DateTime, Utc};
use iced::window;

use activity_tracker_lib::data::payload::Payload;

#[derive(Debug, Clone)]
pub enum MainMessage {
    // Windows
    OpenWindow,
    WindowOpened(window::Id),
    WindowClosed(window::Id),

    // New activity data
    ActivityData(DateTime<Utc>, u32),

    // Tray
    TrayIconClick,
    TrayEvent(String),

    // Window controls
    WindowMinimize(window::Id),
    WindowMaximize(window::Id),
    WindowClose(window::Id),

    // Drag area
    WindowDrag(window::Id),

    // Load task
    FontLoaded(Result<(), iced::font::Error>),

    // Main data
    Payload(Payload)
}
