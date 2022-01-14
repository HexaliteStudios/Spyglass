use std::hash::Hash;

use bevy::{ecs::system::Resource, input::ElementState, prelude::*};

pub fn check_combination<T: Send + Resource + Eq + Copy + Hash>(
    c1: T,
    c2: T,
    krd: &Res<Input<T>>,
) -> Option<ElementState> {
    let c1_result = get_element_state(c1, krd);
    if c1_result == get_element_state(c2, krd) {
        c1_result
    } else {
        None
    }
}

pub fn get_element_state<T: Send + Resource + Eq + Copy + Hash>(
    code: T,
    rd: &Res<Input<T>>,
) -> Option<ElementState> {
    if rd.just_pressed(code) {
        Some(ElementState::Pressed)
    } else if rd.just_released(code) {
        Some(ElementState::Released)
    } else {
        None
    }
}
