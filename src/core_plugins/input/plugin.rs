use crate::prelude::*;
use bevy::{input::ElementState, prelude::*};

pub struct InputPlugin;

// This plugin is responsible for handling both keyboard and controller input.
// Is active only when the game is in the `GameState::Playing` state.

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputEvent>()
            .add_plugin(KeyboardSupportPlugin)
            .add_plugin(ControllerSupportPlugin)
            .add_system(input_test);
    }
}

fn input_test(mut rd: EventReader<InputEvent>) {
    for e in rd.iter() {
        println!("Input received from {} - {}", e.kind, e.binding);
    }
}
