use std::thread::sleep;
use std::io::{Write, stdout};
use std::any::Any;
use std::rc::{Rc, Weak};
use std::cell::{RefCell, Ref, RefMut};
use std::time::Duration;
use crossterm::{queue, Result, Output, style, cursor, input, terminal, ExecutableCommand};
use crossterm::terminal::{ClearType, size};
use crossterm::screen::RawScreen;

mod buffer;
mod input_events;
#[macro_use]
mod ui_element;
pub mod ui_components;
pub use buffer::*;
pub use input_events::*;
pub use ui_element::*;
use crossterm::input::KeyEvent;
use std::collections::vec_deque::IterMut;


pub struct Scene {
    elements: Vec<Rc<RefCell<Box<dyn UiElement>>>>,
    current_focused: usize,
    name: &'static str,
    focused: bool,
}

impl Scene {
    pub fn add_element(&mut self, element: Box<dyn UiElement>){
        self.elements.push(Rc::new(RefCell::new(element)));
    }
    pub fn new(name: &'static str) -> Scene {
        Scene {elements: vec![], name, focused: false, current_focused: 0 }
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
    fn get_focused_element(&mut self) -> &Rc<RefCell<Box<dyn UiElement>>> {
        self.elements.iter()
            .filter(|e| e.borrow().is_focusable()).cycle()
            .nth(self.current_focused).unwrap()
    }
}

impl UiElement for Scene {
    fn update(&mut self, events: &InputEvents) {
        for event in &events.key_events {
            if let KeyEvent::Tab = event {
                self.get_focused_element().borrow_mut().on_focus_removed();
                self.current_focused+=1;
                self.get_focused_element().borrow_mut().on_focus();
            }
        }
        for element in &mut self.elements{
            element.borrow_mut().update(events);
        }
    }

    fn render(&self, buffer: &mut SizedBuffer) {
        for element in &self.elements{
            element.borrow().render(buffer);
        }
    }

    ui_component_impl!();
}

pub struct Console {
    buffer: SizedBuffer,
    scenes: Vec<Scene>,
}

impl Console {

    fn full_render_chars(&self) -> Result<()>{
        stdout().execute(terminal::Clear(ClearType::All)).unwrap();

        for y in 0..self.buffer.height(){
            for x in 0..self.buffer.width(){
                print!("{0}", self.buffer.get_pixel(x,y).content);
            }
            print!("\n");
        }
        Ok(())
    }

    fn update_render_chars(&self, old_buffer: SizedBuffer) -> Result<()> {
        let mut stdout = std::io::stdout();
        for change in self.buffer.compare(&old_buffer) {
            queue!(stdout, cursor::MoveTo(change.position.0, change.position.1),
                style::PrintStyledContent(change.value.to_styled_content()))?;
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
        let (w, h) = size().expect("Failed to get terminal size.");
        Console{ buffer: SizedBuffer::new(w, h), scenes: vec![] }
    }

    pub fn add_scene(&mut self, scene: Scene){
        self.scenes.push(scene);
    }

    pub fn main_loop(&mut self, update_callback: fn(console: &mut Console)){
        let _raw = RawScreen::into_raw_mode();
        let input = input::input();
        let mut reader = input.read_async();
        stdout().execute(terminal::Clear(ClearType::All)).unwrap();

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
