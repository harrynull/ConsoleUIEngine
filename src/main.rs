mod console_ui;
mod gol;

fn main() {

    let mut ui = console_ui::Console::new();
    let _board = gol::GameOfLife{};

    let mut scene = console_ui::Scene::new();
    scene.add_element(Box::new(console_ui::ui_components::Text{ text: "Hello, world!".to_string(), position: (2, 10) }));
    ui.add_scene(scene);
    ui.main_loop();
}
