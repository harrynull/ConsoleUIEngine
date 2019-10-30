use super::super::UiElement;
use super::super::SizedBuffer;
use std::any::Any;

pub struct Text {
    pub text: String,
    pub position: (u16, u16),
    pub name: &'static str
}

impl UiElement for Text {
    fn render(&self, buffer: &mut SizedBuffer) {
        let mut offset = 0;
        for c in self.text.chars() {
            buffer.set_pixel(c, self.position.0 + offset, self.position.1);
            offset += 1;
        }
    }

    fn get_name(&self) -> &str {
        self.name.clone()
    }
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
