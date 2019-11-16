use std::any::Any;

use crossterm::input::KeyEvent;
use crossterm::style;
use crossterm::style::ContentStyle;

use crate::console_ui::ConsoleUpdateInfo;
use crate::console_ui::ui_components::Content;

use super::Label;
use super::super::SizedBuffer;
use super::super::UiElement;

ui_component_struct!(
pub struct Typer {
    pub text: Label,
    pub content: Content,
    pub speed: usize,
    progress: usize,
});

impl Typer {
    pub(crate) fn new(name: &'static str, content: Content, position: (u16, u16), speed: usize) -> Typer {
        Typer {
            name,
            focused: false,
            text: Label::new("", Content::from_string("".to_string()), position),
            content,
            speed,
            progress: 0
        }
    }
}

impl UiElement for Typer {
    fn update(&mut self, console: &mut ConsoleUpdateInfo) {
        if self.progress < self.content.len() * self.speed {
            self.progress += 1;
            self.text.replace_content(self.content.substr(0, self.progress / self.speed));
        }
        self.text.update(console);
    }
    fn render(&self, buffer: &mut SizedBuffer) { self.text.render(buffer); }

    ui_component_impl!();

    fn is_focusable(&self) -> bool { false }
}
