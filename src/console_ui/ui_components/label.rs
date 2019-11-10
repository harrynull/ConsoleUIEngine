use super::super::UiElement;
use super::super::SizedBuffer;
use std::any::Any;
use super::super::StyledChar;
use crossterm::style;
use crossterm::style::{StyledContent, ContentStyle};
use crate::console_ui::ui_components::Content;

pub struct Label {
    pub name: &'static str,
    pub position: (u16, u16),
    focused: bool,
    content: Content,
}

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

impl UiElement for Label {
    fn render(&self, buffer: &mut SizedBuffer) {
        let iter = match &self.content {
            Content::Plain(content, style) => {
                let mut xoffset = 0;
                for c in content.chars() {
                    let mut sc = StyledChar::from_char(c);
                    if let Some(style) = style {
                        sc.style = style.clone();
                    }
                    buffer.set_pixel(&sc, self.position.0 + xoffset, self.position.1);
                    xoffset += 1;
                }
            },
            Content::RichText(c) => {
            },
        };
    }
    ui_component_impl!();
}
