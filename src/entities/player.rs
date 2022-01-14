use bevy::asset::Assets;
use bevy::prelude::*;
use crate::prelude::*;

#[derive(Component)]
pub struct Player;

impl BaseEntity for Player {
    fn spawn(mut commands: Commands, _font_assets: Res<FontAssets>, _block_textures: Res<BlockTextureAssets>, character_textures: Res<CharacterTextureAssets>, _ui_textures: Res<UITextureAssets>, transform: Option<Transform>) {
        commands
            .spawn_bundle(
                SpriteBundle {
                    texture: character_textures.main_character.clone(),
                    transform: transform.unwrap_or_else(default_transform),
                    ..Default::default()
                }
            )
            .insert(Health(Health::maximum()))
            .insert(Player);
    }
}

