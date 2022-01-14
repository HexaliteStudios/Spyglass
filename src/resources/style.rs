use bevy::prelude::*;

pub fn build_style(top: f32, left: f32) -> Style {
    Style {
        position_type: PositionType::Absolute,
        position: Rect {
            top: Val::Px(top),
            left: Val::Px(left),
            ..Default::default()
        },
        ..Default::default()
    }
}