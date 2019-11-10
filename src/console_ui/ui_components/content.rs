use std::thread::current;

use crossterm::style::{Attribute, Color, ContentStyle};

use crate::console_ui::StyledChar;

#[derive(Clone)]
pub enum Content {
    Plain(String, Option<ContentStyle>),
    RichText(Vec<StyledChar>)
}

impl Content{
    pub fn from_string(str: String) -> Content {
        Content::Plain(str, None)
    }
    pub fn from_string_styled(str: String, style: Option<ContentStyle>) -> Content {
        Content::Plain(str, style)
    }

    pub fn from_string_parse_style(str: String) -> Content {
        let mut ret = Vec::new();
        let mut current_style = ContentStyle::new();
        let mut escape = false;
        let mut undo = false;
        for c in str.chars() {
            if escape {
                match c {
                    'U' => {
                        if !undo { current_style.attributes.push(Attribute::Underlined) }
                        else { current_style.attributes.retain(|&e| e!=Attribute::Underlined) }
                    }
                    'r' => current_style.foreground_color = Some(Color::Red),
                    'R' => current_style.background_color = Some(Color::Red),
                    'g' => current_style.foreground_color = Some(Color::Green),
                    'G' => current_style.background_color = Some(Color::Green),
                    'b' => current_style.foreground_color = Some(Color::Blue),
                    'B' => current_style.background_color = Some(Color::Blue),
                    'd' => current_style.foreground_color = Some(Color::Black),
                    'D' => current_style.background_color = Some(Color::Black),
                    'c' => current_style.foreground_color = None,
                    'C' => current_style.background_color = None,
                    _ => {}
                }
                if c == 'u' { undo = true; escape = true; }
                else { undo = false; escape = false;}
                continue;
            }
            if c == '\\' {
                escape = true;
            } else {
                ret.push(StyledChar{ style: current_style.clone(), content: c });
            }
        }
        Content::RichText(ret)
    }

    pub fn insert(&mut self, i: usize, ch: char) {
        match self{
            Content::Plain(c, _s) => { c.insert(i, ch); },
            Content::RichText(c) => { c.insert(i, StyledChar::from_char(ch)) },
        }
    }

    pub fn remove(&mut self, i: usize) {
        match self{
            Content::Plain(c, _s) => { c.remove(i); },
            Content::RichText(c) => { c.remove(i); },
        }
    }

    pub fn len(&self) -> usize {
        match self{
            Content::Plain(c, s) => { c.len() },
            Content::RichText(c) => { c.len() },
        }
    }
}