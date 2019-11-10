use std::any::Any;

use crate::console_ui::ConsoleUpdateInfo;

use super::InputEvents;
use super::SizedBuffer;

pub trait UiElement {
    fn update(&mut self, _console: &mut ConsoleUpdateInfo) {}
    fn render(&self, _buffer: &mut SizedBuffer) {}
    fn get_name(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn is_focusable(&self) -> bool {false}
    fn has_focus(&self) -> bool;
    fn on_focus(&mut self) {}
    fn on_focus_removed(&mut self) {}
}

#[macro_export]
macro_rules! ui_component_struct {
    (pub struct $name:ident { $( $vis:vis $field:ident: $ty:ty ),* $(,)* }) => {
        pub struct $name {
            pub name: &'static str,
            focused: bool,
            $( $vis $field: $ty ),*
        }
    };
}

#[macro_export]
macro_rules! ui_component_impl {
    () => {
        fn get_name(&self) -> &str {
            self.name.clone()
        }
        fn has_focus(&self) -> bool { self.focused }

        fn as_any(&self) -> &dyn Any { self }
        fn as_any_mut(&mut self) -> &mut dyn Any { self }
    };
}