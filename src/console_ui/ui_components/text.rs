use super::super::UiElement;
use super::super::SizedBuffer;
use std::any::Any;
use super::super::StyledChar;
use crossterm::style;
use crossterm::style::{StyledContent, ContentStyle};


ui_component_struct!(
pub struct Text {
    pub content: String,
    pub position: (u16, u16),
    pub text_style: Option<ContentStyle>
});

impl Text {
    pub(crate) fn new(name: &'static str, content: String, position: (u16, u16)) -> Text {
        Text {
            name,
            focused: false,
            content,
            position,
            text_style: None
        }
    }
}

impl UiElement for Text {
    fn render(&self, buffer: &mut SizedBuffer) {
        let mut offset = 0;
        for c in self.content.chars() {
            let mut sc = StyledChar::from_char(c);
            if let Some(style) = &self.text_style {
                sc.style = style.clone();
            }else{
                sc.style.foreground_color = Some(style::Color::Green);
            }
            buffer.set_pixel(&sc, self.position.0 + offset, self.position.1);
            offset += 1;
        }
    }
    ui_component_impl!();
}
