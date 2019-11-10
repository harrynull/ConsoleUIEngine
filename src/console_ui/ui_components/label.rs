use std::any::Any;

use crossterm::style;
use crossterm::style::{ContentStyle, StyledContent};

use crate::console_ui::ui_components::Content;

use super::super::SizedBuffer;
use super::super::StyledChar;
use super::super::UiElement;

ui_component_struct!(
pub struct Label {
    pub position: (u16, u16),
    content: Content,
});

impl Label {
    pub fn new(name: &'static str, content: Content, position: (u16, u16)) -> Label {
        Label {
            name,
            focused: false,
            position,
            content,
        }
    }

    pub fn get_content(&self) -> &Content {
        &self.content
    }

    pub fn get_content_mut(&mut self) -> &mut Content {
        &mut self.content
    }

    pub fn replace_content(&mut self, content: Content) {
        self.content = content;
    }
}

pub fn render_line(buffer: &mut SizedBuffer, content: &Content, position: (u16, u16)) {
    let mut x_offset = 0;
    let iter = match content {
        Content::Plain(content, style) => {
            for c in content.chars() {
                let mut sc = StyledChar::from_char(c);
                if let Some(style) = style {
                    sc.style = style.clone();
                }
                buffer.set_pixel(&sc, position.0 + x_offset, position.1);
                x_offset += 1;
            }
        },
        Content::RichText(content) => {
            for c in content {
                buffer.set_pixel(&c, position.0 + x_offset, position.1);
                x_offset += 1;
            }
        },
    };

}

impl UiElement for Label {
    fn render(&self, buffer: &mut SizedBuffer) {
        render_line(buffer, &self.content, self.position);
    }
    ui_component_impl!();
}
