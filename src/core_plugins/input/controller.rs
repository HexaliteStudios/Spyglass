use bevy::prelude::*;
use crate::prelude::*;

struct SpyglassGamepad(Gamepad);
// ----------------------------------------------------------------------------- //

pub struct ControllerSupportPlugin;

// Plugin responsible for sending controller input events to the main input system.
// This plugin is only active in the `GameState::Playing` state.

impl Plugin for ControllerSupportPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}
