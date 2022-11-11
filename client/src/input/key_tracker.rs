use specs_derive::Component;
use specs::prelude::{Component, VecStorage, NullStorage};

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct KeyboardControlled;

#[derive(Component, Debug, Copy, Clone)]
#[storage(VecStorage)]
pub struct KeyTracker {
    pub up: bool,
    pub down: bool,
    pub right: bool,
    pub left: bool
}
impl KeyTracker {
    pub fn new () -> KeyTracker {
        KeyTracker {
            up: false,
            down: false,
            right: false,
            left: false
        }
    }
}