use std::any::Any;

use crossterm::style;
use crossterm::style::ContentStyle;

use crossterm::input::KeyEvent;
use crossterm::input::MouseEvent::Press;
use crate::ui_components::Content;
use crate::ui_element::UiElement;
use crate::console::ConsoleUpdateInfo;
use crate::ui_components::Content::Plain;
use crate::buffer::SizedBuffer;
use crate::ui_components::Label;

ui_component_struct!(
pub struct Button {
    pub text: Label,
    pressed: bool,
});

impl Button {
    pub fn new(name: &'static str, content: String, position: (u16, u16)) -> Button {
        Button {
            name,
            focused: false,
            text: Label::new("", Content::from_string(content), position),
            pressed: false
        }
    }

    pub fn is_pressed(&self) -> bool { self.pressed }
}

impl UiElement for Button {
    fn update(&mut self, console: &mut ConsoleUpdateInfo) {
        let style = if self.focused {
            Some(ContentStyle {
                foreground_color: Some(style::Color::Black),
                background_color: Some(style::Color::White),
                attributes: vec![]
            })
        } else {
            Some(ContentStyle {
                foreground_color: Some(style::Color::Green),
                background_color: None,
                attributes: vec![]
            })
        };
        if let Plain(_, ref mut s) = &mut self.text.get_content_mut() {
            *s = style;
        }

        self.pressed = false;
        if self.focused {
            for event in &console.get_events().key_events {
                if let KeyEvent::Enter = event {
                    self.pressed = true;
                }
            }
            for event in &console.get_events().mouse_events {
                if let Press(press, x, y) = event {
                    if self.is_clicked(*x, *y) {
                        self.pressed = true;
                    }
                }
            }
        }

        self.text.update(console);
    }
    fn render(&self, buffer: &mut SizedBuffer) {
        self.text.render(buffer);
    }

    fn is_clicked(&self, x: u16, y: u16) -> bool { self.text.is_clicked(x, y) }

    ui_component_impl!();

    fn is_focusable(&self) -> bool { true }

    fn on_focus(&mut self) {
        self.focused = true;
    }

    fn on_focus_removed(&mut self) {
        self.focused = false;
    }
}
