pub mod keyboard;
pub mod mouse;

use crate::input::keyboard::Keyboard;
use crate::input::mouse::Mouse;

#[derive(Copy, Clone)]
pub struct ButtonState {
    pub is_down: bool,
    pub was_down: bool,
}

impl ButtonState {
    pub fn new() -> ButtonState {
        ButtonState {
            is_down: false,
            was_down: false,
        }
    }

    pub fn reset(&mut self) {
        self.is_down = false;
        self.was_down = false;
    }

    pub fn is_down(&self) -> bool {
        self.is_down
    }

    pub fn pressed(&self) -> bool {
        self.is_down && !self.was_down
    }

    pub fn released(&self) -> bool {
        !self.is_down && self.was_down
    }
}

pub struct Input {
    pub mouse: Mouse,
    pub keyboard: Keyboard,
}

impl Input {
    pub fn new() -> Input {
        Input {
            mouse: Mouse::new(),
            keyboard: Keyboard::new(),
        }
    }
}
