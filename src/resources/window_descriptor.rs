use bevy::prelude::*;

pub fn build_window_descriptor() -> WindowDescriptor {
    WindowDescriptor {
        width: 1280.,
        height: 720.,
        title: String::from("Spyglass"),
        ..Default::default()
    }
}