use std::any::Any;
use std::cell::{Ref, RefCell, RefMut};
use std::io::{stdout, Write};
use std::rc::{Rc, Weak};
use std::thread::sleep;
use std::time::Duration;

use crossterm::{cursor, ExecutableCommand, input, Output, queue, Result, style, terminal};
use crossterm::input::KeyEvent;
use crossterm::screen::RawScreen;
use crossterm::terminal::{ClearType, size};

pub use buffer::*;
pub use input_events::*;
pub use ui_element::*;

mod buffer;
mod input_events;
#[macro_use]
mod ui_element;
pub mod ui_components;

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
    pub fn find_child<T>(&self, name: &str) -> Option<&Rc<RefCell<Box<dyn UiElement>>>> where T: UiElement, T: 'static {
        self.elements.iter().find(|e|e.borrow().get_name() == name)
    }
    fn get_focused_element(&mut self) -> Option<&Rc<RefCell<Box<dyn UiElement>>>> {
        self.elements.iter()
            .filter(|e| e.borrow().is_focusable()).cycle()
            .nth(self.current_focused)
    }
}

#[macro_export]
macro_rules! get_child {
    ($scene:ident, $name:expr, $type:ty, $as:ident, $as_borrow:ident) => {
    let $as_borrow = $scene.find_child::<$type>($name).unwrap().borrow();
    let $as = $as_borrow.as_any().downcast_ref::<$type>().unwrap();
    };
}

#[macro_export]
macro_rules! get_child_mut {
    ($scene:ident, $name:expr, $type:ty, $as:ident, $as_borrow:ident) => {
    let mut $as_borrow = $scene.find_child::<$type>($name).unwrap().borrow_mut();
    let mut $as = $as_borrow.as_any_mut().downcast_mut::<$type>().unwrap();
    };
}

#[macro_export]
macro_rules! add_elements {
    [$scene:ident: $($type:ty{$($parameter:expr),*}),+] => {
    $(
    $scene.add_element(Box::new(<$type>::new($($parameter),*)));
    )+
    };
}

impl UiElement for Scene {
    fn update(&mut self, console: &mut ConsoleUpdateInfo) {
        for event in &console.get_events().key_events {
            if let KeyEvent::Tab | KeyEvent::BackTab = event {
                if let Some(e) = self.get_focused_element() {
                    e.borrow_mut().on_focus_removed();
                    if *event == KeyEvent::Tab {
                        self.current_focused += 1
                    }else if self.current_focused > 0 {
                        self.current_focused -= 1
                    }else if self.current_focused == 0 {
                        self.current_focused = self.elements.len() - 1;
                    }
                    self.get_focused_element().unwrap().borrow_mut().on_focus();
                }
            }
        }
        for element in &mut self.elements{
            element.borrow_mut().update(console);
        }
    }

    fn render(&self, buffer: &mut SizedBuffer) {
        for element in &self.elements{
            element.borrow().render(buffer);
        }
    }

    ui_component_impl!();
}

pub struct ConsoleUpdateInfo {
    cursor_pos: (u16, u16),
    input_events: InputEvents,
}

impl ConsoleUpdateInfo {
    pub fn get_events(&self) -> &InputEvents {
        &self.input_events
    }
    pub fn set_cursor(&mut self, new_cursor_pos: (u16, u16)) {
        self.cursor_pos = new_cursor_pos;
    }
}

pub struct Console {
    buffer: SizedBuffer,
    scenes: Vec<Scene>,
    cursor_pos: (u16, u16),
    should_stop: bool,
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
        stdout().execute(cursor::MoveTo(self.cursor_pos.0, self.cursor_pos.1)).unwrap();
    }

    fn update(&mut self, update_info: &mut ConsoleUpdateInfo) {
        if self.scenes.is_empty() {
            return;
        }
        self.get_current_scene_mut().unwrap().update(update_info);
        self.cursor_pos = update_info.cursor_pos;
    }

    pub fn new() -> Console {
        let (w, h) = size().expect("Failed to get terminal size.");
        Console{ buffer: SizedBuffer::new(w, h), scenes: vec![], cursor_pos: (0, 0), should_stop: false }
    }

    pub fn add_scene(&mut self, scene: Scene){
        self.scenes.push(scene);
    }

    pub fn main_loop(&mut self, update_callback: fn(console: &mut Console, update_info: &mut ConsoleUpdateInfo)){
        let _raw = RawScreen::into_raw_mode();
        stdout().execute(terminal::Clear(ClearType::All)).unwrap();
        let input = input::input();
        let mut reader = input.read_async();
        loop{
            let mut update_info = ConsoleUpdateInfo {
                cursor_pos: self.cursor_pos, input_events: InputEvents::new(&mut reader)
            };
            update_callback(self, &mut update_info);
            self.update(&mut update_info);
            self.render();
            if self.should_stop { break; }
            sleep(Duration::from_millis(10));
        }
    }

    pub fn get_current_scene_mut(&mut self) -> Option<&mut Scene> {
        self.scenes.last_mut()
    }

    pub fn get_current_scene(&self) -> Option<&Scene> {
        self.scenes.last()
    }

    pub fn exit(&mut self) {
        self.should_stop = true;
    }
}
