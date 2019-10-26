use super::SizedBuffer;
pub trait UiElement {
    fn update(&self) {}
    fn render(&self, buffer: &mut SizedBuffer);
}
