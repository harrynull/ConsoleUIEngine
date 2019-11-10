use super::super::UiElement;
use super::super::SizedBuffer;
use super::Text;
use std::any::Any;
use super::super::StyledChar;
use crossterm::style;
use crossterm::style::ContentStyle;
use crate::console_ui::ConsoleUpdateInfo;


ui_component_struct!(
pub struct Button {
    pub text: Text,
});

impl Button {
    pub(crate) fn new(name: &'static str, content: String, position: (u16, u16)) -> Button {
        Button {
            name,
            focused: false,
            text: Text::new("", content, position),
        }
    }
}

impl UiElement for Button {
    fn update(&mut self, console: &mut ConsoleUpdateInfo){
        if self.focused {
            self.text.text_style = Some(ContentStyle{
                foreground_color: Some(style::Color::Black),
                background_color: Some(style::Color::White),
                attributes: vec![]
            });
        }else{
            self.text.text_style = Some(ContentStyle{
                foreground_color: Some(style::Color::Green),
                background_color: None,
                attributes: vec![]
            });
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
