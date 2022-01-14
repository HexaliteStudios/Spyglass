use bevy::prelude::*;
use crate::prelude::*;

pub fn build_text(font_assets: Res<FontAssets>, text: &str, size: Option<f32>, color: Option<Color>, style: Style) -> TextBundle {
    TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: String::from(text),
                    style: build_text_style(font_assets, size, color)
                }
            ],
            ..Default::default()
        },
        style,
        ..Default::default()
    }
}