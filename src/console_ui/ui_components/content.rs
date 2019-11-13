use std::thread::current;

use crossterm::style::{Attribute, Color, ContentStyle};

use crate::console_ui::StyledChar;
use std::str::FromStr;

enum StyleType {
    ForegroundColor(Color),
    BackgroundColor(Color),
    Attribute(Attribute)
}

impl StyleType {
    fn from_str(name: &str) -> StyleType {
        match name.to_lowercase().as_str() {
            "bold" => { StyleType::Attribute(Attribute::Bold) }
            "dim" => { StyleType::Attribute(Attribute::Dim) }
            "italic" => { StyleType::Attribute(Attribute::Italic) }
            "underline" => { StyleType::Attribute(Attribute::Underlined) }
            "slow_blink" => { StyleType::Attribute(Attribute::SlowBlink) }
            "rapid_blink" => { StyleType::Attribute(Attribute::RapidBlink) }
            "reverse" => { StyleType::Attribute(Attribute::Reverse) }
            "hidden" => { StyleType::Attribute(Attribute::Hidden) }
            "crossed_out" => { StyleType::Attribute(Attribute::CrossedOut) }
            "fraktur" => { StyleType::Attribute(Attribute::Fraktur) }
            "framed" => { StyleType::Attribute(Attribute::Framed) }
            "encircled" => { StyleType::Attribute(Attribute::Encircled) }
            "overline" => { StyleType::Attribute(Attribute::OverLined) }
            _ => {
                let mut rgb: Option<Color> = None;
                if name[5..].starts_with("rgb"){
                    let rgb_str = &name[9..name.len()-1];
                    let mut res=rgb_str.split(',').map(|e|e.trim().parse::<u8>()
                        .expect(format!("Failed to parse rich text style: RGB not valid: {}",rgb_str.to_string()).as_str()));
                    let r=res.nth(0).unwrap();
                    let g=res.nth(0).unwrap();
                    let b=res.nth(0).unwrap();
                    println!("{} -> {} {} {}",name.to_string(),r,g,b);
                    rgb = Some(Color::Rgb { r, g, b });
                }

                if name.starts_with("fore:") {
                    if let Some(rgb) = rgb{
                        StyleType::ForegroundColor(rgb)
                    } else {
                        StyleType::ForegroundColor(Color::from_str(&name[5..]).unwrap())
                    }
                } else {
                    if let Some(rgb) = rgb {
                        StyleType::BackgroundColor(rgb)
                    } else {
                        StyleType::BackgroundColor(Color::from_str(&name[5..]).unwrap())
                    }
                }
            }
        }
    }
}

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
        let mut style_stack = Vec::new();
        let mut escape = false;
        let mut tag = false;
        let mut current_tag_content = String::new();
        for c in str.chars() {
            if c=='<' {
                if escape { escape = false; }
                else { tag = true; continue; }
            }

            if tag {
                if c=='>' {
                    tag = false;
                    if current_tag_content.starts_with('/'){
                        let style = style_stack.pop().expect("Failed to parse rich text style.");

                        match style {
                            StyleType::ForegroundColor(_) => {
                                current_style.foreground_color = None;
                            },
                            StyleType::BackgroundColor(_) => {
                                current_style.background_color = None;
                            },
                            StyleType::Attribute(attr) => {
                                current_style.attributes.retain(|&e| e!=attr);
                            },
                        }
                    }
                    else {
                        let style = StyleType::from_str(current_tag_content.as_str());
                        match style {
                            StyleType::ForegroundColor(color) => {
                                current_style.foreground_color = Some(color);
                            },
                            StyleType::BackgroundColor(color) => {
                                current_style.background_color = Some(color);
                            },
                            StyleType::Attribute(attr) => {
                                current_style.attributes.push(attr);
                            },
                        }
                        style_stack.push(style);
                    }
                    current_tag_content.clear();
                } else {
                    current_tag_content.push(c);
                }
                continue;
            }

            if c == '\\' {
                escape = true;
                continue;
            }

            ret.push(StyledChar{ style: current_style.clone(), content: c });
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