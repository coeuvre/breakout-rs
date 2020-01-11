use crate::input::ButtonState;

#[derive(Copy, Clone)]
#[repr(usize)]
pub enum Key {
    Left,
    Right,
    Up,
    Down,

    Count,
}

pub struct Keyboard {
    pub keys: [ButtonState; Key::Count as usize],
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            keys: [ButtonState::new(); Key::Count as usize],
        }
    }

    pub fn key(&self, key: Key) -> &ButtonState {
        assert!((key as usize) < (Key::Count as usize));
        &self.keys[key as usize]
    }

    pub fn key_mut(&mut self, key: Key) -> &mut ButtonState {
        assert!((key as usize) < (Key::Count as usize));
        &mut self.keys[key as usize]
    }
}
