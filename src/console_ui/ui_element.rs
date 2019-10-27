use super::SizedBuffer;
use super::InputEvents;
pub trait UiElement {
    fn update(&mut self, _events: &InputEvents) {}
    fn render(&self, buffer: &mut SizedBuffer);
}
