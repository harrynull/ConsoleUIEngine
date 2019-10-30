use super::SizedBuffer;
use super::InputEvents;
use std::any::Any;

pub trait UiElement {
    fn update(&mut self, _events: &InputEvents) {}
    fn render(&self, buffer: &mut SizedBuffer);
    fn get_name(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
