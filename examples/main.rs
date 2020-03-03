use console_ui_engine_null::scene::Scene;
use console_ui_engine_null::console::{ConsoleUpdateInfo, Console};
use crossterm::input::KeyEvent;
use console_ui_engine_null::add_elements;
use console_ui_engine_null::get_child;
use console_ui_engine_null::ui_components::*;
use std::fs;

/*  To generate the rainbow:
    def rainbow(hue):
        (r, g, b) = colorsys.hsv_to_rgb(hue, 1.0, 1.0)
        return (int(255 * r), int(255 * g), int(255 * b))
    res=""
    cur=0
    text="The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog."
    for i in numpy.arange(0, 1, 1/len(text)):
        res+="<back:rgb"+str(rainbow(i))+">"+text[cur]+"</>"
        cur+=1
*/

fn update_callback(scene: &mut Scene, update_info: &mut ConsoleUpdateInfo) {
    if {
        get_child!(scene, "button", Button, button, _button);
        button.is_pressed()
    } {
        update_info.new_scene(second_scene());
    }
}

fn update_callback2(scene: &mut Scene, update_info: &mut ConsoleUpdateInfo) {

}

fn main_callback(console: &mut Console, update_info: &mut ConsoleUpdateInfo) {
    if update_info.get_events().key_events.iter().find(|e| **e==KeyEvent::Esc).is_some() {
        update_info.request_exit();
    }
}

fn first_scene() -> Scene {
    let mut scene = Scene::new("test scene", update_callback);
    add_elements![scene:
        Rectangle {"rectangle", (1, 2), (115, 25)},
        Text {"text", Content::from_string_parse_style(
             fs::read_to_string("example_text.txt").unwrap_or("Failed to load example_text.txt".to_string())
             .replace("\r\n","\n").replace("\n\n","<newline></newline>").replace("\n","").replace("<newline></newline>","\n")),
             (5, 3), (109, 15)},
        Input {"input", "Type your username here".to_string(), (5, 25)},
        Typer {"label", Content::from_string_parse_style("Hello! This is a <fore:yellow>test</fore:yellow> message!".to_string()), (5, 23), 8},
        Checkbox {"checkbox", "I have read and agreed to the above Terms and Conditions".to_string(), (30, 24)},
        Button {"button", "Start".to_string(), (55, 25)},
        FpsIndicator {"fps", (0, 0)}
    ];
    scene
}

fn second_scene() -> Scene {
    let mut scene = Scene::new("test scene", update_callback2);
    add_elements![scene:
        FpsIndicator {"fps", (0, 0)}
    ];
    scene
}

fn main() {
    let mut ui = Console::new();
    ui.add_scene(first_scene());
    ui.main_loop(main_callback);
}
