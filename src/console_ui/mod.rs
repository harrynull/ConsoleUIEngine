use std::thread::sleep;
use std::time::Duration;
use crossterm::{Terminal, execute, queue, Color, PrintStyledFont, Colorize, Goto, Clear, ClearType, Result, Output};

mod buffer;
mod ui_element;
pub mod ui_components;
pub use buffer::*;
pub use ui_element::*;
use std::io::Write;


pub struct Scene {
    elements: Vec<Box<UiElement>>,
}

impl Scene {
    pub fn add_element(&mut self, element: Box<UiElement>){
        self.elements.push(element);
    }
    pub fn new() -> Scene {
        Scene {elements: vec![]}
    }
}

impl UiElement for Scene {
    fn update(&self) {
        for element in &self.elements{
            element.update();
        }
    }

    fn render(&self, buffer: &mut SizedBuffer) {
        for element in &self.elements{
            element.render(buffer);
        }
    }
}

pub struct Console {
    buffer: SizedBuffer,
    scenes: Vec<Scene>,
    terminal: Terminal,
}

impl Console {

    fn full_render_chars(&self) -> Result<()>{
        self.terminal.clear(ClearType::All)?;

        for y in 0..self.buffer.height(){
            for x in 0..self.buffer.width(){
                print!("{0}",self.buffer.get_pixel(x,y));
            }
            print!("\n");
        }
        Ok(())
    }

    fn update_render_chars(&self, old_buffer: SizedBuffer) -> Result<()> {
        let mut stdout = std::io::stdout();
        for change in self.buffer.compare(&old_buffer) {
            queue!(stdout, Goto(change.position.0, change.position.1), Output(change.value.to_string()))?;
        }
        stdout.flush()?;
        Ok(())
    }

    fn render(&mut self) {
        if self.scenes.is_empty() {
            return;
        }
        let old_buffer = self.buffer.clone();
        self.scenes.last().unwrap().render(&mut self.buffer);
        self.update_render_chars(old_buffer);
    }

    fn update(&mut self) {
        if self.scenes.is_empty() {
            return;
        }
        self.scenes.last().unwrap().update();
    }

    pub fn new() -> Console {
        let terminal = Terminal::new();
        let (w, h) = terminal.size().expect("Failed to get terminal size.");
        Console{ buffer: SizedBuffer::new(w, h), scenes: vec![], terminal }
    }

    pub fn add_scene(&mut self, scene: Scene){
        self.scenes.push(scene);
    }

    pub fn main_loop(&mut self){
        loop{
            self.update();
            self.render();
            sleep(Duration::from_millis(100));
        }
    }
}
