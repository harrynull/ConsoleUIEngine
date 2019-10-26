use super::super::UiElement;
use super::super::SizedBuffer;

pub struct Rectangle {
    pub position: (u16, u16),
    pub size: (u16, u16),
    pub fill: bool,
}

static CHARS: [char; 7]  = ['┏','┓','┗','┛','━','┃','█'];

impl UiElement for Rectangle {
    fn render(&self, buffer: &mut SizedBuffer) {
        buffer.set_pixel(CHARS[0], self.position.0, self.position.1); // ┏
        buffer.set_pixel(CHARS[1], self.position.0+self.size.0, self.position.1); // ┓
        buffer.set_pixel(CHARS[2], self.position.0, self.position.1+self.size.1); // ┗
        buffer.set_pixel(CHARS[3], self.position.0+self.size.0, self.position.1+self.size.1); // ┛
        buffer.draw_hline(CHARS[4], self.position.1, self.position.0+1, self.position.0 + self.size.0 - 1); // ━
        buffer.draw_hline(CHARS[4], self.position.1+self.size.1, self.position.0+1, self.position.0 + self.size.0 - 1); // ━
        buffer.draw_vline(CHARS[5], self.position.0, self.position.1+1, self.position.1 + self.size.1 - 1); // ┃
        buffer.draw_vline(CHARS[5], self.position.0+self.size.0, self.position.1+1, self.position.1 + self.size.1 - 1); // ┃
    }
}