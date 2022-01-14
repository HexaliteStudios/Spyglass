use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/GoudyBookletter1911-Regular.ttf")]
    pub goudy_bookletter: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct BlockTextureAssets {
    #[asset(path = "textures/blocks/grass.png")]
    pub grass: Handle<Texture>,
}

#[derive(AssetCollection)]
pub struct CharacterTextureAssets {
    #[asset(path = "textures/characters/main_character.png")]
    pub main_character: Handle<Texture>,
}

#[derive(AssetCollection)]
pub struct UITextureAssets {
    #[asset(path = "textures/ui/warning.png")]
    pub warning: Handle<Texture>,
}
