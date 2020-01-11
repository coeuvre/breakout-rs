use crate::input::ButtonState;
use crate::math::Vec2;

#[derive(Copy, Clone)]
#[repr(usize)]
pub enum Button {
    Left,
    Right,

    Count,
}

pub struct Mouse {
    pub position: Vec2,
    pub buttons: [ButtonState; Button::Count as usize],
}

impl Mouse {
    pub fn new() -> Mouse {
        Mouse {
            position: Vec2::zero(),
            buttons: [ButtonState::new(); Button::Count as usize],
        }
    }

    pub fn button(&self, button: Button) -> &ButtonState {
        assert!((button as usize) < (Button::Count as usize));
        &self.buttons[button as usize]
    }

    pub fn button_mut(&mut self, button: Button) -> &mut ButtonState {
        assert!((button as usize) < (Button::Count as usize));
        &mut self.buttons[button as usize]
    }
}
