use super::super::UiElement;
use super::super::SizedBuffer;
use super::Text;
use crate::console_ui::{InputEvents, ConsoleUpdateInfo};
use std::any::Any;
use crossterm::input::KeyEvent;

ui_component_struct!(
pub struct Input {
    pub text: Text,
});

impl Input {
    pub fn new(name: &'static str, text: String, position: (u16, u16)) -> Input {
        Input {
            name,
            text: Text::new("", text, position),
            focused: false
        }
    }
}

impl UiElement for Input {
    fn update(&mut self, console: &mut ConsoleUpdateInfo) {
        self.text.update(console);
        if self.has_focus() {
            for event in &console.get_events().key_events {
                match event {
                    KeyEvent::Backspace => { self.text.content.pop(); },
                    //KeyEvent::Left => {},
                    //KeyEvent::Right => {},
                    //KeyEvent::Delete => {},
                    KeyEvent::Char(c) => { self.text.content.push(*c); },
                    _ => {}
                }
            }
            let (x,y) = self.text.position;
            console.set_cursor((x+self.text.content.len() as u16,y));
        }
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
