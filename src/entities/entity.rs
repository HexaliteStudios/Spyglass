use bevy::prelude::*;
use crate::assets::*;

pub trait BaseEntity {
    fn spawn(commands: Commands, font_assets: Res<FontAssets>, block_textures: Res<BlockTextureAssets>, character_textures: Res<CharacterTextureAssets>, ui_textures: Res<UITextureAssets>, transform: Option<Transform>);
}

pub fn default_transform() -> Transform {
    Transform::from_translation(Vec3::new(0., 0., 1.))
}