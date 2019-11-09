use super::super::UiElement;
use super::super::SizedBuffer;
use std::any::Any;
use super::super::StyledChar;

ui_component_struct!(
pub struct Rectangle {
    pub position: (u16, u16),
    pub size: (u16, u16),
    pub fill: bool,
});

impl Rectangle {
    pub fn new(name: &'static str, position: (u16, u16), size: (u16, u16)) -> Rectangle {
        Rectangle {
            name,
            focused: false,
            position,
            size,
            fill: false
        }
    }
}
impl UiElement for Rectangle {
    fn render(&self, buffer: &mut SizedBuffer) {
        let CHARS: [StyledChar; 7]  = [
            StyledChar::from_char('┏'),
            StyledChar::from_char('┓'),
            StyledChar::from_char('┗'),
            StyledChar::from_char('┛'),
            StyledChar::from_char('━'),
            StyledChar::from_char('┃'),
            StyledChar::from_char('█')];

        buffer.set_pixel(&CHARS[0], self.position.0, self.position.1); // ┏
        buffer.set_pixel(&CHARS[1], self.position.0+self.size.0, self.position.1); // ┓
        buffer.set_pixel(&CHARS[2], self.position.0, self.position.1+self.size.1); // ┗
        buffer.set_pixel(&CHARS[3], self.position.0+self.size.0, self.position.1+self.size.1); // ┛
        buffer.draw_hline(&CHARS[4], self.position.1, self.position.0+1, self.position.0 + self.size.0 - 1); // ━
        buffer.draw_hline(&CHARS[4], self.position.1+self.size.1, self.position.0+1, self.position.0 + self.size.0 - 1); // ━
        buffer.draw_vline(&CHARS[5], self.position.0, self.position.1+1, self.position.1 + self.size.1 - 1); // ┃
        buffer.draw_vline(&CHARS[5], self.position.0+self.size.0, self.position.1+1, self.position.1 + self.size.1 - 1); // ┃
    }

    ui_component_impl!();
}