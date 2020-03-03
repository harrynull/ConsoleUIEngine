use std::any::Any;

use crossterm::style::ContentStyle;

use crate::buffer::{SizedBuffer, StyledChar};
use crate::ui_components::{Content, render_line};
use crate::ui_components::Content::{Plain, RichText};
use crate::ui_element::UiElement;

pub enum WordWrap {
    Normal, BreakWord
}

ui_component_struct!(
pub struct Text {
    pub position: (u16, u16),
    pub size: (u16, u16),
    pub word_wrap: WordWrap,
    content: Vec<Content>,
    raw_content: Content,
});

fn break_line_str_break(content: String, size: (u16, u16), style: Option<ContentStyle>) -> Vec<Content> {
    let (w, _h) = size;
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

fn break_line_rich_text_break(content: Vec<StyledChar>, size: (u16, u16)) -> Vec<Content> {
    let (w, _h) = size;
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

fn break_line_str_normal(content: String, size: (u16, u16), style: Option<ContentStyle>) -> Vec<Content> {
    let w = size.0 as usize;
    let mut ret = Vec::new();
    let mut current_line = String::new();
    let mut current_word = String::new();
    for c in content.chars() {
        let char_val = c;
        if char_val != '\n' { current_word.push( c); }
        if char_val == ' ' || char_val == '\n' { // a new word
            while current_line.len() + current_word.len() > w { // if the current line cannot contain this word
                if current_word.len() > w { // break the word if the word is too long to be contained in one line
                    let (cur, nxt) = current_word.split_at_mut(w-current_line.len());
                    current_line.push_str(&mut cur.to_string());
                    current_word = nxt.to_string();
                }
                ret.push(Plain(current_line.clone(), style.clone())); // a new line required
                current_line.clear();
            }
            current_line.push_str(&mut current_word);
        }
        if char_val =='\n' { // new line required
            current_line.push_str(&mut current_word);
            ret.push(Plain(current_line.clone(), style.clone()));
            current_line.clear();
        }
    }
    if !current_word.is_empty() {
        current_line.push_str(&mut current_word);
    }
    if !current_line.is_empty() {
        ret.push(Plain(current_line.clone(), style.clone()));
    }
    ret
}

fn break_line_rich_text_normal(content: Vec<StyledChar>, size: (u16, u16)) -> Vec<Content> {
    let w = size.0 as usize;
    let mut ret = Vec::new();
    let mut current_line = Vec::new();
    let mut current_word = Vec::new();
    for c in content {
        let char_val = c.content.clone();
        if char_val != '\n' { current_word.push( c); }
        if char_val == ' ' || char_val == '\n' { // a new word
            while current_line.len() + current_word.len() > w { // if the current line cannot contain this word
                if current_word.len() > w { // break the word if the word is too long to be contained in one line
                    let (cur, nxt) = current_word.split_at_mut(w-current_line.len());
                    current_line.append(&mut cur.to_vec());
                    current_word = nxt.to_vec();
                }
                ret.push(RichText(current_line.clone())); // a new line required
                current_line.clear();
            }
            current_line.append(&mut current_word);
        }
        if char_val =='\n' { // new line required
            current_line.append(&mut current_word);
            ret.push(RichText(current_line.clone()));
            current_line.clear();
        }
    }
    if !current_word.is_empty() {
        current_line.append(&mut current_word);
    }
    if !current_line.is_empty() {
        ret.push(RichText(current_line.clone()));
    }
    ret
}

fn wrap_content(content: Content, wrap_type: WordWrap, size: (u16, u16)) -> Vec<Content> {
    match content {
        Content::Plain(c, style) => { match wrap_type {
            WordWrap::Normal => break_line_str_normal(c, size, style),
            WordWrap::BreakWord => break_line_str_break(c, size, style),
        }},
        Content::RichText(c) => { match wrap_type {
            WordWrap::Normal => break_line_rich_text_normal(c, size),
            WordWrap::BreakWord => break_line_rich_text_break(c, size),
        }},
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

    fn is_clicked(&self, x: u16, y: u16) -> bool {
        x >= self.position.0 && x < self.position.0 + self.size.0
            && y >= self.position.1 && y <= self.position.1 + self.content.len() as u16
    }

    ui_component_impl!();
}
