use crossterm::{KeyEvent, MouseEvent, InputEvent, AsyncReader};

pub struct InputEvents{
    pub key_events: Vec<KeyEvent>,
    pub mouse_events: Vec<MouseEvent>,
}

impl InputEvents {
    pub fn new(reader: &mut AsyncReader) -> InputEvents {
        let mut input_events = InputEvents{ key_events: vec![], mouse_events: vec![] };
        while let Some(event) = reader.next() {
            match event {
                InputEvent::Keyboard(event) => {
                    input_events.key_events.push(event);
                }
                InputEvent::Mouse(event) => {
                    input_events.mouse_events.push(event);
                }
                _ => {}
            }
        }
        input_events
    }
}