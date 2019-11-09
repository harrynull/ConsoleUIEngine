use super::super::UiElement;
use super::super::SizedBuffer;
use super::Text;
use crate::console_ui::InputEvents;
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
    fn update(&mut self, events: &InputEvents) {
        self.text.update(events);
        if self.has_focus() {
            for event in &events.key_events {
                match event {
                    KeyEvent::Backspace => { self.text.content.pop(); },
                    //KeyEvent::Left => {},
                    //KeyEvent::Right => {},
                    //KeyEvent::Delete => {},
                    KeyEvent::Char(c) => { self.text.content.push(*c); },
                    _ => {}
                }
            }
            // TODO: cursor position
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
