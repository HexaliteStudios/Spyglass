use std::collections::HashMap;
use bevy::prelude::*;
use either::*;
use crate::core_plugins::input::bindings::ControllerCombination::SingleAxis;

pub enum MovementBindingKind {
    Left,
    Right,
    Up,
    Down,
}

