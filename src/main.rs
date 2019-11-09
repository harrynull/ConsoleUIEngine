use crate::console_ui::{Console, Scene};
use crate::console_ui::ui_components::{Input, Text};
use std::cell::RefMut;

mod console_ui;
mod gol;


fn update_callback(console: &mut Console) {
    let scene = console.get_current_scene_mut().unwrap();
    let input = scene.find_child::<Input>("input").unwrap().text.content.len();
    let mut text: RefMut<Text> = scene.find_child_mut::<Text>("text").unwrap();
    text.content = input.to_string();
}

fn main() {
    let mut ui = console_ui::Console::new();
    let _board = gol::GameOfLife{};

    let mut scene = console_ui::Scene::new("test scene");
    scene.add_element(Box::new(console_ui::ui_components::Text::new(
        "text","Hello, world!".to_string(),(5, 10)
    )));
    scene.add_element(Box::new(console_ui::ui_components::Input::new(
        "input", "Type something...".to_string(), (5, 11)
    )));
    scene.add_element(Box::new(console_ui::ui_components::Rectangle::new(
        "rectangle", (1, 2), (25, 15)
    )));
    ui.add_scene(scene);
    ui.main_loop(update_callback);
}
