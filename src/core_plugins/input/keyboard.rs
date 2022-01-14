use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use crate::prelude::*;

pub struct KeyboardSupportPlugin;

// Plugin responsible for sending keyboard input events to the main input system.
// This plugin is only active in the `GameState::Playing` state.

impl Plugin for KeyboardSupportPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(listen_to_keyboard_input);
    }
}

fn listen_to_keyboard_input(mut reader: EventReader<KeyboardInput>) {
    use bevy::input::ElementState;

    for event in reader.iter() {
        let e = InputEvent::new()
    }
}
