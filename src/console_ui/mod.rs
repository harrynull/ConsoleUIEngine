use std::thread::sleep;
use std::time::Duration;
use crossterm::{input, Terminal, queue, Goto, ClearType, Result, Output, RawScreen};

mod buffer;
mod input_events;
#[macro_use]
mod ui_element;
pub mod ui_components;
pub use buffer::*;
pub use input_events::*;
pub use ui_element::*;
use std::io::Write;
use std::any::Any;
use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};


pub struct Scene {
    elements: Vec<Rc<RefCell<Box<dyn UiElement>>>>,
    name: &'static str
}

impl Scene {
    pub fn add_element(&mut self, element: Box<dyn UiElement>){
        self.elements.push(Rc::new(RefCell::new(element)));
    }
    pub fn new(name: &'static str) -> Scene {
        Scene {elements: vec![], name }
    }

    pub fn find_child<T>(&self, name: &str) -> Option<Ref<T>> where T: UiElement, T: 'static {
        self.elements.iter().find(|e|e.borrow().get_name() == name)
            .map(|e| Ref::map(e.borrow(),
                         |e| e.as_any().downcast_ref::<T>().unwrap()))
    }
    pub fn find_child_mut<T>(&mut self, name: &str) -> Option<RefMut<T>> where T: UiElement, T: 'static {
        self.elements.iter_mut().find(|e|e.borrow().get_name() == name)
            .map(|e| RefMut::map(e.borrow_mut(),
                    |e| e.as_any_mut().downcast_mut::<T>().unwrap()))
    }


}

impl UiElement for Scene {
    fn update(&mut self, events: &InputEvents) {
        for element in &mut self.elements{
            element.borrow_mut().update(events);
        }
    }

    fn render(&self, buffer: &mut SizedBuffer) {
        for element in &self.elements{
            element.borrow().render(buffer);
        }
    }
    fn get_name(&self) -> &str{ self.name.clone() }
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
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
        self.buffer = SizedBuffer::new(self.buffer.width(), self.buffer.height());
        self.scenes.last().unwrap().render(&mut self.buffer);
        self.update_render_chars(old_buffer).unwrap();
    }

    fn update(&mut self, events: &InputEvents) {
        if self.scenes.is_empty() {
            return;
        }
        self.scenes.last_mut().unwrap().update(events);
    }

    pub fn new() -> Console {
        let terminal = Terminal::new();
        let (w, h) = terminal.size().expect("Failed to get terminal size.");
        Console{ buffer: SizedBuffer::new(w, h), scenes: vec![], terminal }
    }

    pub fn add_scene(&mut self, scene: Scene){
        self.scenes.push(scene);
    }

    pub fn main_loop(&mut self, update_callback: fn(console: &mut Console)){
        let _raw = RawScreen::into_raw_mode();
        let input = input();
        let mut reader = input.read_async();
        self.terminal.clear(ClearType::All).unwrap();

        loop{
            update_callback(self);
            self.update(&InputEvents::new(&mut reader));
            self.render();
            sleep(Duration::from_millis(50));
        }
    }

    pub fn get_current_scene_mut(&mut self) -> Option<&mut Scene> {
        self.scenes.last_mut()
    }

    pub fn get_current_scene(&mut self) -> Option<&Scene> {
        self.scenes.last()
    }
}
