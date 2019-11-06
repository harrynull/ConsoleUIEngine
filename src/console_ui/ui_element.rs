use super::SizedBuffer;
use super::InputEvents;
use std::any::Any;

pub trait UiElement {
    fn update(&mut self, _events: &InputEvents) {}
    fn render(&self, buffer: &mut SizedBuffer) {}
    fn get_name(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[macro_export]
macro_rules! ui_component_struct {
    (pub struct $name:ident { $( pub $field:ident: $ty:ty ),* $(,)* }) => {
        pub struct $name {
            pub name: &'static str,
            $( pub $field: $ty ),*
        }
    };
}

#[macro_export]
macro_rules! ui_component_impl {
    () => {
        fn get_name(&self) -> &str {
            self.name.clone()
        }
        fn as_any(&self) -> &dyn Any { self }
        fn as_any_mut(&mut self) -> &mut dyn Any { self }
    };
}