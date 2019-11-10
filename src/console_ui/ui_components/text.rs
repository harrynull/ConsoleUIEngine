use super::super::UiElement;
use super::super::SizedBuffer;
use std::any::Any;
use super::super::StyledChar;
use crossterm::style;
use crossterm::style::{StyledContent, ContentStyle};
use crate::console_ui::ui_components::{Content, render_line};
use crate::console_ui::ui_components::Content::{Plain, RichText};

pub enum WordWrap {
    Normal, BreakWord
}

pub struct Text {
    pub name: &'static str,
    pub position: (u16, u16),
    pub size: (u16, u16),
    pub word_wrap: WordWrap,
    focused: bool,
    content: Vec<Content>,
    raw_content: Content,
}

fn break_line_str(content: String, wrap_type: WordWrap, size: (u16, u16), style: Option<ContentStyle>) -> Vec<Content> {
    let (w,h) = size;
    let mut cnt = 0;
    let mut ret = Vec::new();
    let mut current_line = String::new();
    for c in content.chars() {
        cnt+=1;
        if c != '\n' { current_line.push(c); }
        if cnt == w || c=='\n' {
            cnt = 0;
            ret.push(Plain(current_line.clone(), style.clone()));
            current_line.clear();
        }
    }
    if !current_line.is_empty() {
        ret.push(Plain(current_line.clone(), style.clone()));
    }
    ret
}

fn break_line_rich_text(content: Vec<StyledChar>, wrap_type: WordWrap, size: (u16, u16)) -> Vec<Content> {
    let (w,h) = size;
    let mut cnt = 0;
    let mut ret = Vec::new();
    let mut current_line = Vec::new();
    for c in content {
        let char_val = c.content.clone();
        cnt+=1;
        if char_val != '\n' { current_line.push( c); }
        if cnt == w || char_val =='\n'{
            cnt = 0;
            ret.push(RichText(current_line.clone()));
            current_line.clear();
        }
    }
    if !current_line.is_empty() {
        ret.push(RichText(current_line.clone()));
    }
    ret
}

fn wrap_content(content: Content, wrap_type: WordWrap, size: (u16, u16)) -> Vec<Content> {
    match content {
        Content::Plain(c, style) => { break_line_str(c, wrap_type, size, style) },
        Content::RichText(c) => { break_line_rich_text(c, wrap_type, size) },
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
        let mut y_offset = 0;
        for line in &self.content {
            let mut pos = self.position;
            pos.1 += y_offset;
            render_line(buffer, &line, pos);
            y_offset += 1;
        }
    }
    ui_component_impl!();
}
