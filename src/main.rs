use crate::console_ui::{Console, Scene};
use crate::console_ui::ui_components::{Input, Text};
use std::cell::RefMut;

mod console_ui;
mod gol;


fn update_callback(console: &mut Console) {
    let scene = console.get_current_scene_mut().unwrap();
    let input = scene.find_child::<Input>("input").unwrap().text.text.clone();
    let mut text: RefMut<Text> = scene.find_child_mut::<Text>("text").unwrap();
    text.text = (input.parse::<i32>().unwrap_or_default() * 2).to_string();
}

fn main() {
    let mut ui = console_ui::Console::new();
    let _board = gol::GameOfLife{};

    let mut scene = console_ui::Scene::new("test scene");
    scene.add_element(Box::new(console_ui::ui_components::Text{
        text: "Hello, world!".parse().unwrap(),
        position: (5, 10),
        name: "text" })
    );
    scene.add_element(Box::new(console_ui::ui_components::Input{
        text: console_ui::ui_components::Text{
            text: "Type something...".parse().unwrap(),
            position: (5, 11),
            name: ""
        },
        focus: true,
        name: "input"
    }));
    scene.add_element(Box::new(console_ui::ui_components::Rectangle {
        size: (25, 15),
        position: (1, 2),
        fill: false,
        name: "rectangle"
    }));
    ui.add_scene(scene);
    ui.main_loop(update_callback);
}
