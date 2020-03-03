use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crossterm::input::KeyEvent;
use crossterm::input::MouseEvent::Press;

use crate::buffer::SizedBuffer;
use crate::console::ConsoleUpdateInfo;
use crate::ui_element::UiElement;

pub struct Scene {
    elements: Vec<Rc<RefCell<Box<dyn UiElement>>>>,
    name: &'static str,
    current_focused: usize,
    nothing_focused: bool,
    focused: bool,
    focusable_elements_count: usize,
    update_callback: fn(scene: &mut Scene, update_info: &mut ConsoleUpdateInfo),
}

impl Scene {
    pub fn add_element(&mut self, element: Box<dyn UiElement>){
        if element.is_focusable() {
            self.focusable_elements_count += 1;
        }
        self.elements.push(Rc::new(RefCell::new(element)));
    }
    pub fn new(name: &'static str, update_callback: fn(scene: &mut Scene, update_info: &mut ConsoleUpdateInfo)) -> Scene {
        Scene {elements: vec![], name, focused: false, current_focused: 0, nothing_focused: true, focusable_elements_count: 0, update_callback }
    }
    pub fn find_child<T>(&self, name: &str) -> Option<&Rc<RefCell<Box<dyn UiElement>>>> where T: UiElement, T: 'static {
        self.elements.iter().find(|e|e.borrow().get_name() == name)
    }
    fn get_nth_focusable_element(&mut self, n: usize) -> Option<&Rc<RefCell<Box<dyn UiElement>>>> {
        self.elements.iter()
            .filter(|e| e.borrow().is_focusable()).cycle()
            .nth(n)
    }
    fn get_focused_element(&mut self) -> Option<&Rc<RefCell<Box<dyn UiElement>>>> {
        self.get_nth_focusable_element(self.current_focused)
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
        if self.nothing_focused && self.focusable_elements_count != 0 {
            self.get_nth_focusable_element(0).unwrap().borrow_mut().on_focus(); // focus the first element
        }

        // Processing keyboard events related to focus
        for event in &console.get_events().key_events {
            if let KeyEvent::Tab | KeyEvent::BackTab = event {
                if let Some(e) = self.get_focused_element() {
                    e.borrow_mut().on_focus_removed();
                    if *event == KeyEvent::Tab {
                        self.current_focused += 1
                    }else if self.current_focused > 0 {
                        self.current_focused -= 1
                    }else if self.current_focused == 0 {
                        self.current_focused = self.focusable_elements_count - 1;
                    }
                    self.get_focused_element().unwrap().borrow_mut().on_focus();
                }
            }
        }

        // Processing mouse click events related to focus
        let old_focused = self.current_focused;
        let mut focusable_id = 0;
        for event in &console.get_events().mouse_events {
            if let Press(_, x, y) = event {
                for element in &mut self.elements {
                    let focusable = element.borrow().is_focusable();
                    if focusable && element.borrow().is_clicked(*x, *y) {
                        self.current_focused = focusable_id;
                        break;
                    }
                    if focusable {
                        focusable_id += 1;
                    }
                }
            }
            if old_focused != self.current_focused {
                if let Some(e) = self.get_nth_focusable_element(old_focused) {
                    e.borrow_mut().on_focus_removed();
                }
                self.get_focused_element().unwrap().borrow_mut().on_focus();
                break;
            }
        }

        (self.update_callback)(self, console);
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
