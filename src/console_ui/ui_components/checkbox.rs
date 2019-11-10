use super::super::UiElement;
use super::super::SizedBuffer;
use std::any::Any;
use super::super::StyledChar;
use crossterm::style;
use super::Text;
use crossterm::input::KeyEvent;
use crate::console_ui::ConsoleUpdateInfo;
use crossterm::style::ContentStyle;


ui_component_struct!(
pub struct Checkbox {
    pub text: Text,
    pub content: String,
    pub selected: bool,
});

impl Checkbox {
    pub(crate) fn new(name: &'static str, content: String, position: (u16, u16)) -> Checkbox {
        Checkbox {
            name,
            focused: false,
            selected: false,
            text: Text::new("", "".to_string(), position),
            content
        }
    }
}

impl UiElement for Checkbox {
    fn update(&mut self, console: &mut ConsoleUpdateInfo) {
        let mut updated = false;
        if self.has_focus() {
            for event in &console.get_events().key_events {
                if let KeyEvent::Enter = event {
                    self.selected = !self.selected;
                    updated = true;
                }
            }
            self.text.text_style = Some(ContentStyle{
                foreground_color: Some(style::Color::Green),
                background_color: None,
                attributes: vec![style::Attribute::Underlined]
            });
        }else{
            self.text.text_style = None;
        }
        self.text.content = (if self.selected { "[x] " } else { "[ ] " }).to_string() + &self.content;
        self.text.update(console);
    }
    fn render(&self, buffer: &mut SizedBuffer) { self.text.render(buffer); }

    ui_component_impl!();

    fn is_focusable(&self) -> bool { true }

    fn on_focus(&mut self) {
        self.focused = true;
    }

    fn on_focus_removed(&mut self) {
        self.focused = false;
    }
}
