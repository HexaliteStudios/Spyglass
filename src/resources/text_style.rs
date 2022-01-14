use bevy::prelude::*;
use crate::prelude::*;

pub fn build_text_style(font_assets: Res<FontAssets>, font_size: Option<f32>, color: Option<Color>) -> TextStyle {
    TextStyle {
        font: font_assets.goudy_bookletter.clone(),
        font_size: font_size.unwrap_or(100.),
        color: color.unwrap_or(Color::WHITE)
    }
}