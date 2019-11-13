use std::cell::RefMut;
use std::rc::Rc;

use crossterm::input::KeyEvent;

use crate::console_ui::{Console, ConsoleUpdateInfo, Scene};
use crate::console_ui::ui_components::*;

mod console_ui;

static mut PROGRESS: usize = 0;
static SPEED: usize = 8;
static TEXT: &str = "Hello! This is a test message!";
static LONG_TEXT: &str = "                                            <bold>Terms and Conditions</bold>\n\
            <fore:red>Red <underline>underline</></><back:blue>blue <fore:black>black</> blue</> \
            <reverse>The quick brown fox jumps over the lazy dog.</reverse> \\<force\nnewline>\
            <fore:rgb(255,182,193)>RGB Color Test! (255,182,193)</> The quick brown fox jumps over the lazy dog. \
            <fore:Blue>The quick brown fox</> <fore:Dark_Blue>jumps</> over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. \
            The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. \
            The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. \
            The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. \
            The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. \
            The quick brown fox jumps over the lazy dog. longwordendwithanewlinesymbollongwordendwithanewlinesymbol\n\
Try to use up rest of the line: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa \
            \n\n<underline>Paragraph 2</underline>\n\
            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent posuere, lacus ac mattis blandit, odio erat mollis turpis, id\
            convallis velit magna nec ligula. Praesent nec lorem aliquet, eleifend erat in, interdum enim. Etiam lectus dui, consectetur eget\
            pulvinar vel, gravida in magna. Praesent vitae ipsum massa. Duis eu erat eget nisl viverra maximus vel a turpis.";

fn update_callback(console: &mut Console, update_info: &mut ConsoleUpdateInfo) {
    for event in &update_info.get_events().key_events {
        if let KeyEvent::Esc = event { console.exit(); }
    }
    let scene = console.get_current_scene_mut().unwrap();
    //get_child!(scene, "input", Input, input, _input);
    //get_child!(scene, "input2", Input, input2, _input2);
    get_child_mut!(scene, "label", Label, label, _label);

    if unsafe { PROGRESS } < TEXT.len()*SPEED {
        unsafe { PROGRESS +=1; }
        let current = unsafe { PROGRESS };
        label.replace_content(Content::from_string(TEXT[..current/SPEED].to_string()));
    }
}

fn main() {
    let mut ui = console_ui::Console::new();
    let mut scene = console_ui::Scene::new("test scene");
    add_elements![scene:
        Rectangle {"rectangle", (1, 2), (115, 25)},
        Text {"text",Content::from_string_parse_style(LONG_TEXT.to_string()), (5, 3), (109, 15)},
        Input {"input", "Type your username here".to_string(), (5, 25)},
        Label {"label",Content::from_string("Hello, world!".to_string()), (5, 23)},
        Checkbox {"checkbox", "I have read and agreed to the above Terms and Conditions".to_string(), (30, 24)},
        Button {"button", "Start".to_string(), (55, 25)}
    ];
    ui.add_scene(scene);
    ui.main_loop(update_callback);
}
