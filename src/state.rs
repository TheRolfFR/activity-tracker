use iced::window;

#[derive(Debug, Clone)]
pub enum MainMessage {
    // Windows
    OpenWindow,
    WindowOpened(window::Id),
    WindowClosed(window::Id),
    ScaleInputChanged(window::Id, String),
    ScaleChanged(window::Id, String),
    TitleChanged(window::Id, String),

    // Tray
    TrayEvent(String),

    // Window controls
    WindowMinimize(window::Id),
    WindowMaximize(window::Id),
    WindowClose(window::Id)
}
