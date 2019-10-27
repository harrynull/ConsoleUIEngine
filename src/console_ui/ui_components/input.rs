use super::super::UiElement;
use super::super::SizedBuffer;
use super::Text;
use crate::console_ui::InputEvents;
use crossterm::KeyEvent;

pub struct Input {
    pub text: Text,
    pub focus: bool,
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
}
