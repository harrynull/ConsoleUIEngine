use crate::console_ui::{Console, Scene};
use crate::console_ui::ui_components::{Input, Text};
use std::cell::RefMut;
use std::rc::Rc;

mod console_ui;
mod gol;

fn update_callback(console: &mut Console) {
    let scene = console.get_current_scene_mut().unwrap();

    get_child!(scene, "input", Input, input, input_borrow);
    get_child!(scene, "input2", Input, input2, input2_borrow);
    get_child_mut!(scene, "text", Text, text, text_borrow);

    text.content = (input.text.content.parse::<i32>().unwrap_or(0) + input2.text.content.parse::<i32>().unwrap_or(0)).to_string();
}

fn main() {
    let mut ui = console_ui::Console::new();
    let _board = gol::GameOfLife{};

    let mut scene = console_ui::Scene::new("test scene");
    scene.add_element(Box::new(console_ui::ui_components::Rectangle::new(
        "rectangle", (1, 2), (25, 15)
    )));
    scene.add_element(Box::new(console_ui::ui_components::Text::new(
        "text","Hello, world!".to_string(),(5, 10)
    )));
    scene.add_element(Box::new(console_ui::ui_components::Input::new(
        "input", "Type something...".to_string(), (5, 11)
    )));
    scene.add_element(Box::new(console_ui::ui_components::Input::new(
        "input2", "Another Input!".to_string(), (5, 12)
    )));
    ui.add_scene(scene);
    ui.main_loop(update_callback);
}
