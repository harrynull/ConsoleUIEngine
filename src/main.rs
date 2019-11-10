use crate::console_ui::{Console, Scene, ConsoleUpdateInfo};
use crate::console_ui::ui_components::{Input, Text, Content, Label};
use std::cell::RefMut;
use std::rc::Rc;
use crossterm::input::KeyEvent;

mod console_ui;
mod gol;

static mut PROGRESS: usize = 0;
static SPEED: usize = 8;
static TEXT: &str = "Hello! This is a test message!";

fn update_callback(console: &mut Console, update_info: &mut ConsoleUpdateInfo) {
    for event in &update_info.get_events().key_events {
        if let KeyEvent::Esc = event { console.exit(); }
    }
    let scene = console.get_current_scene_mut().unwrap();
    get_child!(scene, "input", Input, input, _input);
    get_child!(scene, "input2", Input, input2, _input2);
    get_child_mut!(scene, "text", Label, text, _text);

    if unsafe { PROGRESS } < TEXT.len()*SPEED {
        unsafe { PROGRESS +=1; }
        let current = unsafe { PROGRESS };
        text.replace_content(Content::from_string(TEXT[..current/SPEED].to_string()));
    }
}

fn main() {
    let mut ui = console_ui::Console::new();
    let _board = gol::GameOfLife{};

    let mut scene = console_ui::Scene::new("test scene");
    scene.add_element(Box::new(console_ui::ui_components::Rectangle::new(
        "rectangle", (1, 2), (110, 25)
    )));
    scene.add_element(Box::new(console_ui::ui_components::Label::new(
        "text",Content::from_string("Hello, world!".to_string()),(5, 10)
    )));
    scene.add_element(Box::new(console_ui::ui_components::Input::new(
        "input", "Type something...".to_string(), (5, 11)
    )));
    scene.add_element(Box::new(console_ui::ui_components::Input::new(
        "input2", "Another Input!".to_string(), (5, 12)
    )));
    scene.add_element(Box::new(console_ui::ui_components::Button::new(
        "button", "OK".to_string(), (5, 15)
    )));
    scene.add_element(Box::new(console_ui::ui_components::Checkbox::new(
        "checkbox", "select 1".to_string(), (5, 16)
    )));
    scene.add_element(Box::new(console_ui::ui_components::Checkbox::new(
        "checkbox2", "select 2".to_string(), (5, 17)
    )));
    ui.add_scene(scene);
    ui.main_loop(update_callback);
}
