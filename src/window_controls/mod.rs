use iced::{
    alignment::{Horizontal, Vertical},
    widget::{
        button, row,
        svg::{self, Handle, Svg},
        Container,
    },
    window, Background, Color, Length, Renderer, Size,
};
use iced_core::{widget::Tree, Widget};
use iced_fluent_theme::button::medium;
use std::sync::LazyLock;

use crate::handle;
use crate::state::MainMessage;
use crate::Element;

handle!(MINIMIZE_ICON, "win/minimize.svg");
handle!(MAXIMIZE_ICON, "win/maximize.svg");
handle!(MAXIMIZE_RESTORE_ICON, "win/maximize_restore.svg");
handle!(CLOSE_ICON, "win/close.svg");

#[derive(Clone, Copy)]
enum ControlType {
    Minimize,
    Maximize(bool),
    Close,
}

struct Control<'a> {
    content: Element<'a, MainMessage>,
    control_type: ControlType,
    message: MainMessage,
    hovered: Option<bool>,
}

impl<'a> Control<'a> {
    pub(crate) fn new(control_type: ControlType, message: MainMessage) -> Self {
        Self {
            content: transparent_button(control_type, false, message.clone()),
            control_type,
            message,
            hovered: None, // I don't know
        }
    }
}

impl<'a> Widget<MainMessage, iced_fluent_theme::Theme, Renderer> for Control<'a> {
    fn size(&self) -> iced::Size<Length> {
        self.content.as_widget().size()
    }

    fn layout(
        &mut self,
        tree: &mut iced_core::widget::Tree,
        renderer: &Renderer,
        limits: &iced_core::layout::Limits,
    ) -> iced_core::layout::Node {
        self.content
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn children(&self) -> Vec<iced_core::widget::Tree> {
        vec![iced_core::widget::Tree::new(&self.content)]
    }

    fn draw(
        &self,
        tree: &iced_core::widget::Tree,
        renderer: &mut Renderer,
        theme: &iced_fluent_theme::Theme,
        style: &iced_core::renderer::Style,
        layout: iced_core::Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        )
    }

    fn update(
        &mut self,
        tree: &mut iced_core::widget::Tree,
        event: &iced::Event,
        layout: iced_core::Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced_core::Clipboard,
        shell: &mut iced_core::Shell<'_, MainMessage>,
        viewport: &iced::Rectangle,
    ) {
        let current_hovered = cursor.position_over(layout.bounds()).is_some();
        let redraw = match self.hovered {
            Some(previous_hovered) => previous_hovered != current_hovered,
            None => true,
        };

        if redraw {
            self.content =
                transparent_button(self.control_type, current_hovered, self.message.clone());
            self.hovered = Some(current_hovered);
            shell.request_redraw();
        }

        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn size_hint(&self) -> Size<Length> {
        self.size()
    }

    fn tag(&self) -> iced_core::widget::tree::Tag {
        iced_core::widget::tree::Tag::stateless()
    }

    fn state(&self) -> iced_core::widget::tree::State {
        iced_core::widget::tree::State::None
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn operate(
        &mut self,
        tree: &mut iced_core::widget::Tree,
        layout: iced_core::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced_core::widget::Operation,
    ) {
        self.content
            .as_widget_mut()
            .operate(&mut tree.children[0], layout, renderer, operation);
    }

    fn mouse_interaction(
        &self,
        tree: &iced_core::widget::Tree,
        layout: iced_core::Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        viewport: &iced::Rectangle,
        renderer: &Renderer,
    ) -> iced_core::mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut iced_core::widget::Tree,
        layout: iced_core::Layout<'b>,
        renderer: &Renderer,
        viewport: &iced::Rectangle,
        translation: iced::Vector,
    ) -> Option<iced_core::overlay::Element<'b, MainMessage, iced_fluent_theme::Theme, Renderer>>
    {
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a> From<Control<'a>> for Element<'a, MainMessage> {
    fn from(widget: Control<'a>) -> Self {
        Self::new(widget)
    }
}

fn transparent_button<'a>(
    ctype: ControlType,
    is_hovered: bool,
    message: MainMessage,
) -> Element<'a, MainMessage> {
    let opacity = if is_hovered { 1.0 } else { 0.8956 };

    let icon = match &ctype {
        ControlType::Minimize => MINIMIZE_ICON.clone(),
        ControlType::Maximize(restore) => {
            if *restore {
                MAXIMIZE_RESTORE_ICON.clone()
            } else {
                MAXIMIZE_ICON.clone()
            }
        }
        ControlType::Close => CLOSE_ICON.clone(),
    };

    let icon_color = Color::WHITE;

    let icon: Element<'a, MainMessage> = match &ctype {
        // fix for minimize icon not rendered because 1x10, empty button with same white background fill as svg color
        ControlType::Minimize => button("")
            .width(10)
            .height(1)
            .style(move |_, _| button::Style {
                background: Some(Background::Color(icon_color.scale_alpha(opacity))),
                ..Default::default()
            })
            .into(),
        _ => Svg::new(icon)
            .width(10)
            .style(move |_, _| svg::Style {
                color: Some(icon_color),
            })
            .opacity(opacity)
            .into(),
    };

    let container = Container::new(icon)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .height(Length::Fill)
        .width(Length::Fill);

    medium(container)
        .style(move |_, status| button::Style {
            background: match (ctype.clone(), status) {
                (ControlType::Close, button::Status::Hovered) => {
                    Some(iced::Background::Color(Color::from_rgb8(196, 43, 28)))
                }
                (_, button::Status::Hovered) => {
                    Some(iced::Background::Color(Color::WHITE.scale_alpha(0.20)))
                }
                (_, _) => None,
            },
            ..Default::default()
        })
        .width(46)
        .height(32)
        .padding(0)
        .on_press(message)
        .into()
}

pub struct WindowControls {
    show_minimize: bool,
    show_maximized: Option<bool>, // true if Some, and store state inside
    show_close: bool,
    id: window::Id,
}

impl WindowControls {
    pub fn empty(id: window::Id) -> Self {
        Self {
            show_minimize: false,
            show_maximized: None,
            show_close: false,
            id,
        }
    }

    pub fn all(id: window::Id, maximize_state: bool) -> Self {
        Self::empty(id)
            .with_minimize()
            .with_maximize(maximize_state)
            .with_close()
    }

    pub fn with_close(mut self) -> Self {
        self.show_close = true;
        self
    }

    pub fn with_minimize(mut self) -> Self {
        self.show_minimize = true;
        self
    }

    pub fn with_maximize(mut self, maximize_state: bool) -> Self {
        self.show_maximized = Some(maximize_state);
        self
    }
}

impl<'a> From<WindowControls> for Element<'a, MainMessage> {
    fn from(widget: WindowControls) -> Self {
        row![
            widget.show_minimize.then_some(Control::new(
                ControlType::Minimize,
                MainMessage::WindowMinimize(widget.id)
            )),
            widget.show_maximized.map(|state| Control::new(
                ControlType::Maximize(state),
                MainMessage::WindowMaximize(widget.id)
            )),
            widget.show_close.then_some(Control::new(
                ControlType::Close,
                MainMessage::WindowClose(widget.id)
            )),
        ]
        .into()
    }
}
