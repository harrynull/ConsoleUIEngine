use super::super::UiElement;
use super::super::SizedBuffer;
use super::Text;
use crate::console_ui::InputEvents;
use crossterm::KeyEvent;
use std::any::Any;

pub struct Input {
    pub text: Text,
    pub focus: bool,
    pub name: &'static str
}

impl UiElement for Input {
    fn update(&mut self, events: &InputEvents) {
        for event in &events.key_events {
            match event {
                KeyEvent::Backspace => { self.text.text.pop(); },
                //KeyEvent::Left => {},
                //KeyEvent::Right => {},
                //KeyEvent::Delete => {},
                KeyEvent::Char(c) => { self.text.text.push(*c); },
                _ => {}
            }
        }
        // TODO: cursor position
        // TODO: focus
        self.text.update(events);
    }
    fn render(&self, buffer: &mut SizedBuffer) {
        self.text.render(buffer);
    }
    fn get_name(&self) -> &str {
        self.name.clone()
    }
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
