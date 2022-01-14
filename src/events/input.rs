use crate::core_plugins::*;
use bevy::input::ElementState;

#[derive(strum_macros::Display)]
pub enum InputKind {
    Controller,
    Keyboard,
}
pub struct InputEvent {
    pub binding: ActionBindingKind,
    pub state: ElementState,
    pub kind: InputKind,
}

impl InputEvent {
    pub fn new(binding: ActionBindingKind, state: ElementState, kind: InputKind) -> Self {
        InputEvent {
            binding,
            state,
            kind,
        }
    }
}
