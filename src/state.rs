use iced::window;

#[derive(Debug, Clone)]
pub enum MainMessage {
    // Windows
    OpenWindow,
    WindowOpened(window::Id),
    WindowClosed(window::Id),

    // Tray
    TrayIconClick,
    TrayEvent(String),

    // Window controls
    WindowMinimize(window::Id),
    WindowMaximize(window::Id),
    WindowClose(window::Id),

    // Drag area
    WindowDrag(window::Id),
}
