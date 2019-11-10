use super::super::UiElement;
use super::super::SizedBuffer;
use std::any::Any;
use super::super::StyledChar;
use crossterm::style;
use crossterm::style::{StyledContent, ContentStyle};
use crate::console_ui::ui_components::Content;

pub enum WordWrap {
    Normal, BreakWord
}

enum WrappedContent {
    Plain(Vec<String>, Option<ContentStyle>),
    RichText(Vec<Vec<StyledChar>>)
}

pub struct Text {
    pub name: &'static str,
    pub position: (u16, u16),
    pub size: (u16, u16),
    pub word_wrap: WordWrap,
    focused: bool,
    content: WrappedContent,
    raw_content: Content,
}

fn break_line_str(content: String, wrap_type: WordWrap, size: (u16, u16)) -> Vec<String> {
    let (w,h) = size;
    let mut cnt = 0;
    let mut ret = Vec::new();
    let mut current_line = String::new();
    for c in content.chars() {
        cnt+=1;
        current_line.push(c);
        if cnt == w {
            cnt = 0;
        }
    }
    ret
}

fn break_line_rich_text(content: Vec<StyledChar>, wrap_type: WordWrap, size: (u16, u16)) -> Vec<Vec<StyledChar>> {
    let (w,h) = size;
    let mut cnt = 0;
    let mut ret = Vec::new();
    let mut current_line = Vec::new();
    for c in content {
        cnt+=1;
        current_line.push( c);
        if cnt == w {
            cnt = 0;
        }
    }
    ret
}

fn wrap_content(content: Content, wrap_type: WordWrap, size: (u16, u16)) -> WrappedContent {
    match content {
        Content::Plain(c, style) => {
            WrappedContent::Plain(break_line_str(c, wrap_type, size), style)
        },
        Content::RichText(c) => {
            WrappedContent::RichText(break_line_rich_text(c, wrap_type, size))
        },
    }
}

impl Text {
    pub fn new(name: &'static str, content: Content, position: (u16, u16), size: (u16, u16)) -> Text {
        Text {
            name,
            focused: false,
            position,
            size,
            word_wrap: WordWrap::Normal,
            content: wrap_content(content.clone(), WordWrap::Normal, size),
            raw_content: content,
        }
    }

    pub fn get_content(&self) -> &Content {
        &self.raw_content
    }

    pub fn replace_content(&mut self, content: Content) {
        self.raw_content = content.clone();
        self.content = wrap_content(content, WordWrap::Normal, self.size);
    }
}

impl UiElement for Text {
    fn render(&self, buffer: &mut SizedBuffer) {

        let iter = match &self.content {
            WrappedContent::Plain(c, style) => {
                let mut yoffset = 0;
                for line in c {
                    let mut xoffset = 0;
                    for c in line.chars() {
                        let mut sc = StyledChar::from_char(c);
                        if let Some(style) = style {
                            sc.style = style.clone();
                        }
                        buffer.set_pixel(&sc, self.position.0 + xoffset, self.position.1 + yoffset);
                        xoffset += 1;
                    }
                    yoffset += 1;
                }
            },
            WrappedContent::RichText(c) => {
            },
        };
    }
    ui_component_impl!();
}
