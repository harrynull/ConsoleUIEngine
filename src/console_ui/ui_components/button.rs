use super::super::UiElement;
use super::super::SizedBuffer;
use std::any::Any;
use crossterm::style;
use crossterm::style::ContentStyle;
use crate::console_ui::ConsoleUpdateInfo;
use crate::console_ui::ui_components::{Label, Content};
use crate::console_ui::ui_components::Content::Plain;


ui_component_struct!(
pub struct Button {
    pub text: Label,
});

impl Button {
    pub(crate) fn new(name: &'static str, content: String, position: (u16, u16)) -> Button {
        Button {
            name,
            focused: false,
            text: Label::new("", Content::from_string(content), position),
        }
    }
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
        self.text.update(console);
    }
    fn render(&self, buffer: &mut SizedBuffer) {
        self.text.render(buffer);
    }
    ui_component_impl!();

    fn is_focusable(&self) -> bool { true }

    fn on_focus(&mut self) {
        self.focused = true;
    }

    fn on_focus_removed(&mut self) {
        self.focused = false;
    }
}
