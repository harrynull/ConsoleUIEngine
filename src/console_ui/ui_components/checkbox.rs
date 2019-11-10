use super::super::UiElement;
use super::super::SizedBuffer;
use std::any::Any;
use crossterm::style;
use super::Label;
use crossterm::input::KeyEvent;
use crate::console_ui::ConsoleUpdateInfo;
use crossterm::style::ContentStyle;
use crate::console_ui::ui_components::Content;


ui_component_struct!(
pub struct Checkbox {
    pub text: Label,
    pub content: String,
    pub selected: bool,
});

impl Checkbox {
    pub(crate) fn new(name: &'static str, content: String, position: (u16, u16)) -> Checkbox {
        Checkbox {
            name,
            focused: false,
            selected: false,
            text: Label::new("", Content::from_string("".to_string()), position),
            content
        }
    }
}

impl UiElement for Checkbox {
    fn update(&mut self, console: &mut ConsoleUpdateInfo) {
        let mut updated = false;
        let mut text_style = None;
        if self.has_focus() {
            for event in &console.get_events().key_events {
                if let KeyEvent::Enter = event {
                    self.selected = !self.selected;
                    updated = true;
                }
            }
            text_style = Some(ContentStyle{
                foreground_color: Some(style::Color::Green),
                background_color: None,
                attributes: vec![style::Attribute::Underlined]
            });
        }
        self.text.replace_content(Content::from_string_styled(
            (if self.selected { "[x] " } else { "[ ] " }).to_string() + &self.content,
            text_style));
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
