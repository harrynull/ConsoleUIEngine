use super::super::UiElement;
use super::super::SizedBuffer;

pub struct Text {
    pub text: String,
    pub position: (u16, u16),
}

impl UiElement for Text {
    fn render(&self, buffer: &mut SizedBuffer) {
        let mut offset = 0;
        for c in self.text.chars() {
            buffer.set_pixel(c, self.position.0 + offset, self.position.1);
            offset += 1;
        }
    }
}