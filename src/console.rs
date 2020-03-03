use crate::input_events::InputEvents;
use crate::scene::Scene;
use std::io::{stdout, Write};
use crossterm::{cursor, ExecutableCommand, input, Output, queue, Result, style, terminal};
use crossterm::terminal::{ClearType, size};
use std::thread::sleep;
use std::time::Duration;
use crossterm::screen::RawScreen;
use crate::buffer::SizedBuffer;
use crate::ui_element::UiElement;

#[cfg(windows)] extern crate winapi;

#[cfg(windows)]
fn disable_quick_edit() {
    use winapi::um::consoleapi::SetConsoleMode;
    use winapi::um::wincon::{ENABLE_EXTENDED_FLAGS, ENABLE_WINDOW_INPUT, ENABLE_MOUSE_INPUT};
    use winapi::um::winbase::STD_INPUT_HANDLE;
    use winapi::um::processenv::GetStdHandle;
    unsafe {
        let handle = GetStdHandle(STD_INPUT_HANDLE);
        SetConsoleMode(handle, ENABLE_EXTENDED_FLAGS);
        SetConsoleMode(handle, ENABLE_WINDOW_INPUT | ENABLE_MOUSE_INPUT);
    };
}

#[cfg(not(windows))]
fn disable_quick_edit() {
}

pub struct ConsoleUpdateInfo {
    cursor_pos: (u16, u16),
    input_events: InputEvents,
    exit: bool,
    new_scene: Option<Scene>,
}

impl ConsoleUpdateInfo {
    pub fn get_events(&self) -> &InputEvents {
        &self.input_events
    }
    pub fn set_cursor(&mut self, new_cursor_pos: (u16, u16)) {
        self.cursor_pos = new_cursor_pos;
    }
    pub fn request_exit(&mut self) { self.exit = true; }
    pub fn new_scene(&mut self, scene: Scene) { self.new_scene = Some(scene); }
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
        disable_quick_edit();
        let _raw = RawScreen::into_raw_mode();
        stdout().execute(terminal::Clear(ClearType::All)).unwrap();
        let input = input::input();
        let mut reader = input.read_async();
        loop{
            let mut update_info = ConsoleUpdateInfo {
                cursor_pos: self.cursor_pos,
                input_events: InputEvents::new(&mut reader),
                exit: false,
                new_scene: None
            };
            update_callback(self, &mut update_info);
            self.update(&mut update_info);
            self.render();
            if self.should_stop || update_info.exit { break; }
            if let Some(scene) = update_info.new_scene {
                self.add_scene(scene);
            }
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
