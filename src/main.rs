use std::cell::RefMut;
use std::rc::Rc;
use std::fs;
use std::any::Any;

use crossterm::input::{KeyEvent, MouseEvent};
use rand::prelude::*;

use crate::console_ui::{Console, ConsoleUpdateInfo, Scene, UiElement, SizedBuffer, StyledChar};
use crate::console_ui::ui_components::*;
use crossterm::style::{ContentStyle, Color};

mod console_ui;

ui_component_struct!(
pub struct GameOfLife {
    size: (usize, usize),
    map: Vec<u8>
});
impl GameOfLife {
    pub fn get_cell(&self, x: usize, y: usize) -> u8 {
        return self.map[y * self.size.0 + x]
    }

    pub fn get_cell_mut(&mut self, x: usize, y: usize) -> &mut u8 {
        return &mut self.map[y * self.size.0 + x]
    }

    fn live_neighbors(&self, coord: (usize, usize)) -> i32 {
        let mut ret = 0;
        let height = self.size.1;
        let width = self.size.0;

        if coord.0 + 1 >= 0 && coord.0 + 1 < width && coord.1 + 1 >= 0 && coord.1 + 1 < height {
            if self.get_cell(coord.0 + 1, coord.1 + 1) & 1 == 1 {
                ret += 1;
            }
        }
        if coord.0 + 1 >= 0 && coord.0 + 1 < width && coord.1 >= 1 && coord.1 < height + 1 {
            if self.get_cell(coord.0 + 1, coord.1 - 1) & 1 == 1 {
                ret += 1;
            }
        }
        if coord.0 >= 1 && coord.0 < width + 1 && coord.1 + 1 >= 0 && coord.1 + 1 < height {
            if self.get_cell(coord.0 - 1, coord.1 + 1)  & 1 == 1 {
                ret += 1;
            }
        }
        if coord.0 >= 1 && coord.0 < width + 1 && coord.1 >= 1 && coord.1 < height + 1 {
            if self.get_cell(coord.0 - 1, coord.1 - 1)  & 1 == 1 {
                ret += 1;
            }
        }
        if coord.0 + 1 >= 0 && coord.0 + 1 < width && coord.1 >= 0 && coord.1 < height {
            if self.get_cell(coord.0 + 1, coord.1)  & 1 == 1 {
                ret += 1;
            }
        }
        if coord.0 >= 1 && coord.0 < width + 1 && coord.1 >= 0 && coord.1 < height {
            if self.get_cell(coord.0 - 1, coord.1)  & 1 == 1 {
                ret += 1;
            }
        }
        if coord.0 >= 0 && coord.0 < width && coord.1 + 1 >= 0 && coord.1 + 1 < height {
            if self.get_cell(coord.0, coord.1 + 1)  & 1 == 1 {
                ret += 1;
            }
        }
        if coord.0 >= 0 && coord.0 < width && coord.1 >= 1 && coord.1 < height + 1 {
            if self.get_cell(coord.0, coord.1 - 1)  & 1 == 1 {
                ret += 1;
            }
        }
        ret
    }

    fn will_live(&self, coord: (usize, usize)) -> bool {
        let num_alive_nei = self.live_neighbors(coord);
        if num_alive_nei < 2 || num_alive_nei > 3 {
            false
        } else if (num_alive_nei == 2 || num_alive_nei == 3) && self.get_cell(coord.0, coord.1) == 1 {
            true
        } else if num_alive_nei == 3 && self.get_cell(coord.0, coord.1) == 0 {
            true
        } else { false }
    }

    fn update_board(&mut self) {
        let height = self.size.1;
        let width = self.size.0;

        for j in 0..height {
            for i in 0..width {
                if self.will_live((i,j)) {
                    *self.get_cell_mut(i,j) |= 2;
                }
            }
        }

        for j in 0..height {
            for i in 0..width {
                *self.get_cell_mut(i,j) >>= 1;
            }
        }
    }
}

fn color(i: usize, j: usize, size: (usize, usize)) -> Color {
    let i = i as f32 - (size.0 as f32)/2.0;
    let j = j as f32 - (size.1 as f32)/2.0;
    let a = i.atan2(j)/2.0;
    let b =2.0*(-1.0 as f32).acos()/3.0;

    Color::Rgb{
        r: ((a).cos().powi(2)*255.0) as u8,
        g: ((a-b).cos().powi(2)*255.0) as u8,
        b: ((a+b).cos().powi(2)*255.0) as u8
    }
}

impl GameOfLife {
    pub fn new(name: &'static str, size: (usize, usize)) -> GameOfLife {
        GameOfLife {
            name,
            focused: true,
            size,
            map: vec![0; (size.0+1)*(size.1+1)]
        }
    }
}

impl UiElement for GameOfLife {
    fn update(&mut self, console: &mut ConsoleUpdateInfo) {
        /*
        let mut rng = rand::thread_rng();
        for i in 0..self.size.0 {
            for j in 0..self.size.1 {
                if rng.gen_range(0, 100) == 1 {
                    *self.get_cell_mut(i,j) = 1;
                }
            }
        }*/

        for e in &console.get_events().mouse_events {
            if let MouseEvent::Hold(x, y) = e {
                *self.get_cell_mut(*x as usize, *y as usize) = 1;
            }
        }
        self.update_board();
    }

    fn render(&self, buffer: &mut SizedBuffer) {
        let height = self.size.1;
        let width = self.size.0;
        for j in 0..height {
            for i in 0..width {
                if self.get_cell(i, j) == 1 {
                    buffer.set_pixel(&StyledChar {
                        style: ContentStyle {
                            foreground_color: None,
                            background_color: Some(color(i, j, self.size)),
                            attributes: vec![]
                        },
                        content: ' '
                    }, i as u16, j as u16);
                }
            }
        }
    }
    ui_component_impl!();
}

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
    get_child_mut!(scene, "gol", GameOfLife, gol, _gol);
}

fn main_callback(console: &mut Console, update_info: &mut ConsoleUpdateInfo) {
    if update_info.get_events().key_events.iter().find(|e| **e==KeyEvent::Esc).is_some() {
        update_info.request_exit();
    }
}

fn first_scene() -> Scene {
    let mut scene = console_ui::Scene::new("test scene", update_callback);
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
    let mut scene = console_ui::Scene::new("test scene", update_callback2);
    add_elements![scene:
        GameOfLife {"gol", (118*2, 67*2)},
        FpsIndicator {"fps", (0, 0)}
    ];
    scene
}

fn main() {
    let mut ui = console_ui::Console::new();
    ui.add_scene(first_scene());
    ui.main_loop(main_callback);
}
