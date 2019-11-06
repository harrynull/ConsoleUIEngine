use super::super::UiElement;
use super::super::SizedBuffer;
use std::any::Any;
use super::super::StyledChar;
use crossterm::style;


ui_component_struct!(
pub struct Text {
    pub text: String,
    pub position: (u16, u16),
});

impl UiElement for Text {
    fn render(&self, buffer: &mut SizedBuffer) {
        let mut offset = 0;
        for c in self.text.chars() {
            let mut sc = StyledChar::from_char(c);
            sc.style.foreground_color = Some(style::Color::Green);
            buffer.set_pixel(&sc, self.position.0 + offset, self.position.1);
            offset += 1;
        }
    }
    ui_component_impl!();
}
