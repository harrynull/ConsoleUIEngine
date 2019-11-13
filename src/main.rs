use std::cell::RefMut;
use std::rc::Rc;

use crossterm::input::KeyEvent;

use crate::console_ui::{Console, ConsoleUpdateInfo, Scene};
use crate::console_ui::ui_components::*;

mod console_ui;

static mut PROGRESS: usize = 0;
static SPEED: usize = 8;
static TEXT: &str = "Hello! This is a test message!";
/*
To generate the rainbow:

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
static LONG_TEXT: &str = "                                            <bold>Terms and Conditions</bold>\n\
            <fore:red>Red <underline>underline</></><back:blue>blue <fore:black>black</> blue</> \
            <reverse>The quick brown fox jumps over the lazy dog.</reverse> \\<force\nnewline>\
            <fore:rgb(255,182,193)>RGB Color Test! (255,182,193)</> The quick brown fox jumps over the lazy dog. \
            <fore:Blue>The quick brown fox</> <fore:Dark_Blue>jumps</> over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. \
<back:rgb(255, 0, 0)>T</><back:rgb(255, 11, 0)>h</><back:rgb(255, 22, 0)>e</><back:rgb(255, 34, 0)> </>\
<back:rgb(255, 45, 0)>q</><back:rgb(255, 57, 0)>u</><back:rgb(255, 68, 0)>i</><back:rgb(255, 79, 0)>c</>\
<back:rgb(255, 91, 0)>k</><back:rgb(255, 102, 0)> </><back:rgb(255, 114, 0)>b</><back:rgb(255, 125, 0)>r</>\
<back:rgb(255, 137, 0)>o</><back:rgb(255, 148, 0)>w</><back:rgb(255, 159, 0)>n</><back:rgb(255, 171, 0)> </>\
<back:rgb(255, 182, 0)>f</><back:rgb(255, 194, 0)>o</><back:rgb(255, 205, 0)>x</><back:rgb(255, 216, 0)> </>\
<back:rgb(255, 228, 0)>j</><back:rgb(255, 239, 0)>u</><back:rgb(255, 251, 0)>m</><back:rgb(247, 255, 0)>p</>\
<back:rgb(235, 255, 0)>s</><back:rgb(224, 255, 0)> </><back:rgb(213, 255, 0)>o</><back:rgb(201, 255, 0)>v</>\
<back:rgb(190, 255, 0)>e</><back:rgb(178, 255, 0)>r</><back:rgb(167, 255, 0)> </><back:rgb(156, 255, 0)>t</>\
<back:rgb(144, 255, 0)>h</><back:rgb(133, 255, 0)>e</><back:rgb(121, 255, 0)> </><back:rgb(110, 255, 0)>l</>\
<back:rgb(98, 255, 0)>a</><back:rgb(87, 255, 0)>z</><back:rgb(76, 255, 0)>y</><back:rgb(64, 255, 0)> </>\
<back:rgb(53, 255, 0)>d</><back:rgb(41, 255, 0)>o</><back:rgb(30, 255, 0)>g</><back:rgb(19, 255, 0)>.</>\
<back:rgb(7, 255, 0)> </><back:rgb(0, 255, 3)>T</><back:rgb(0, 255, 15)>h</><back:rgb(0, 255, 26)>e</>\
<back:rgb(0, 255, 38)> </><back:rgb(0, 255, 49)>q</><back:rgb(0, 255, 60)>u</><back:rgb(0, 255, 72)>i</>\
<back:rgb(0, 255, 83)>c</><back:rgb(0, 255, 95)>k</><back:rgb(0, 255, 106)> </><back:rgb(0, 255, 117)>b</>\
<back:rgb(0, 255, 129)>r</><back:rgb(0, 255, 140)>o</><back:rgb(0, 255, 152)>w</><back:rgb(0, 255, 163)>n</>\
<back:rgb(0, 255, 175)> </><back:rgb(0, 255, 186)>f</><back:rgb(0, 255, 197)>o</><back:rgb(0, 255, 209)>x</>\
<back:rgb(0, 255, 220)> </><back:rgb(0, 255, 232)>j</><back:rgb(0, 255, 243)>u</><back:rgb(0, 255, 255)>m</>\
<back:rgb(0, 243, 255)>p</><back:rgb(0, 232, 255)>s</><back:rgb(0, 220, 255)> </><back:rgb(0, 209, 255)>o</>\
<back:rgb(0, 197, 255)>v</><back:rgb(0, 186, 255)>e</><back:rgb(0, 175, 255)>r</><back:rgb(0, 163, 255)> </>\
<back:rgb(0, 152, 255)>t</><back:rgb(0, 140, 255)>h</><back:rgb(0, 129, 255)>e</><back:rgb(0, 117, 255)> </>\
<back:rgb(0, 106, 255)>l</><back:rgb(0, 95, 255)>a</><back:rgb(0, 83, 255)>z</><back:rgb(0, 72, 255)>y</>\
<back:rgb(0, 60, 255)> </><back:rgb(0, 49, 255)>d</><back:rgb(0, 38, 255)>o</><back:rgb(0, 26, 255)>g</>\
<back:rgb(0, 15, 255)>.</><back:rgb(0, 3, 255)> </><back:rgb(7, 0, 255)>T</><back:rgb(19, 0, 255)>h</>\
<back:rgb(30, 0, 255)>e</><back:rgb(41, 0, 255)> </><back:rgb(53, 0, 255)>q</><back:rgb(64, 0, 255)>u</>\
<back:rgb(76, 0, 255)>i</><back:rgb(87, 0, 255)>c</><back:rgb(98, 0, 255)>k</><back:rgb(110, 0, 255)> </>\
<back:rgb(121, 0, 255)>b</><back:rgb(133, 0, 255)>r</><back:rgb(144, 0, 255)>o</><back:rgb(156, 0, 255)>w</>\
<back:rgb(167, 0, 255)>n</><back:rgb(178, 0, 255)> </><back:rgb(190, 0, 255)>f</><back:rgb(201, 0, 255)>o</>\
<back:rgb(213, 0, 255)>x</><back:rgb(224, 0, 255)> </><back:rgb(235, 0, 255)>j</><back:rgb(247, 0, 255)>u</>\
<back:rgb(255, 0, 251)>m</><back:rgb(255, 0, 239)>p</><back:rgb(255, 0, 228)>s</><back:rgb(255, 0, 216)> </>\
<back:rgb(255, 0, 205)>o</><back:rgb(255, 0, 194)>v</><back:rgb(255, 0, 182)>e</><back:rgb(255, 0, 171)>r</>\
<back:rgb(255, 0, 159)> </><back:rgb(255, 0, 148)>t</><back:rgb(255, 0, 137)>h</><back:rgb(255, 0, 125)>e</>\
<back:rgb(255, 0, 114)> </><back:rgb(255, 0, 102)>l</><back:rgb(255, 0, 91)>a</><back:rgb(255, 0, 79)>z</>\
<back:rgb(255, 0, 68)>y</><back:rgb(255, 0, 57)> </><back:rgb(255, 0, 45)>d</><back:rgb(255, 0, 34)>o</>\
<back:rgb(255, 0, 22)>g</><back:rgb(255, 0, 11)>.</>\
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
