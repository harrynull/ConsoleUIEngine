use std::any::Any;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::buffer::SizedBuffer;
use crate::console::ConsoleUpdateInfo;
use crate::ui_components::Content;
use crate::ui_components::Label;
use crate::ui_element::UiElement;

ui_component_struct!(
pub struct FpsIndicator {
    pub position: (u16, u16),
    label: Label,
    last_update: u64,
    last_fps: u16
});

impl FpsIndicator {
    pub fn new(name: &'static str, position: (u16, u16)) -> FpsIndicator {
        FpsIndicator {
            name,
            focused: false,
            position,
            label: Label::new("", Content::from_string("".to_string()), position),
            last_update: 0,
            last_fps: 0
        }
    }

    pub fn get_fps(&self) -> u16 {
        self.last_fps
    }

}

impl UiElement for FpsIndicator {
    fn update(&mut self, _console: &mut ConsoleUpdateInfo) {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
        let time = since_the_epoch.as_secs() * 1000 +
            since_the_epoch.subsec_nanos() as u64 / 1_000_000;
        self.last_fps = (1000 / (time - self.last_update)) as u16;
        self.last_update = time;
        self.label.replace_content(Content::from_string("FPS: ".to_string()+self.last_fps.to_string().as_str()));
    }
    fn render(&self, buffer: &mut SizedBuffer) {
        self.label.render(buffer);
    }
    ui_component_impl!();
}
