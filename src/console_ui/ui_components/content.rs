use crossterm::style::ContentStyle;
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

    pub fn insert(&mut self, i: usize, ch: char) {
        match self{
            Content::Plain(c, s) => { c.insert(i, ch); },
            Content::RichText(c) => { c.insert(i, StyledChar::from_char(ch)) },
        }
    }

    pub fn remove(&mut self, i: usize) {
        match self{
            Content::Plain(c, s) => { c.remove(i); },
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