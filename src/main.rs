mod console_ui;
mod gol;

fn main() {

    let mut ui = console_ui::Console::new();
    let _board = gol::GameOfLife{};

    let mut scene = console_ui::Scene::new();
    scene.add_element(Box::new(console_ui::ui_components::Text{ text: "Hello, world!".to_string(), position: (5, 10) }));
    scene.add_element(Box::new(console_ui::ui_components::Input{
        text: console_ui::ui_components::Text{
            text: "Type something...".to_string(),
            position: (5, 11)},
        focus: true
    }));
    scene.add_element(Box::new(console_ui::ui_components::Rectangle { size: (25, 15), position: (1, 2), fill: false }));
    ui.add_scene(scene);
    ui.main_loop();
}
