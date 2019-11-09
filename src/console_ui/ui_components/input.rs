use super::super::UiElement;
use super::super::SizedBuffer;
use super::Text;
use crate::console_ui::{InputEvents, ConsoleUpdateInfo};
use std::any::Any;
use crossterm::input::KeyEvent;

ui_component_struct!(
pub struct Input {
    pub text: Text,
    pub cursor_pos: usize,
});

impl Input {
    pub fn new(name: &'static str, text: String, position: (u16, u16)) -> Input {
        let text_len = text.len();
        Input {
            name,
            text: Text::new("", text, position),
            focused: false,
            cursor_pos: text_len
        }
    }
}

impl UiElement for Input {
    fn update(&mut self, console: &mut ConsoleUpdateInfo) {
        self.text.update(console);
        if self.has_focus() {
            for event in &console.get_events().key_events {
                match event {
                    KeyEvent::Backspace => {
                        if self.cursor_pos > 0 {
                            self.text.content.remove(self.cursor_pos - 1);
                            self.cursor_pos-=1;
                        }
                    },
                    KeyEvent::Delete => {
                        if self.cursor_pos < self.text.content.len() {
                            self.text.content.remove(self.cursor_pos);
                        }
                    },
                    KeyEvent::Left => {
                        if self.cursor_pos > 0 {
                            self.cursor_pos-=1;
                        }
                    },
                    KeyEvent::Right => {
                        if self.cursor_pos < self.text.content.len() {
                            self.cursor_pos+=1;
                        }
                    },
                    KeyEvent::Char(c) => {
                        self.text.content.insert(self.cursor_pos, *c);
                        self.cursor_pos+=1;
                    },
                    _ => {}
                }
            }
            let (x,y) = self.text.position;
            console.set_cursor((x + self.cursor_pos as u16,y));
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
