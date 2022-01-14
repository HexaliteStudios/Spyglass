use bevy::prelude::*;
use crate::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/GoudyBookletter1911-Regular.ttf")]
    pub goudy_bookletter: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct BlockTextureAssets {
    #[asset(path = "textures/blocks/grass.png")]
    pub grass: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct CharacterTextureAssets {
    #[asset(path = "textures/characters/main_character.png")]
    pub main_character: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct UITextureAssets {
    #[asset(path = "textures/ui/warning.png")]
    pub warning: Handle<Image>,
}

pub struct AssetsPlugin;

// This plugin is responsible for loading the game assets during the `GameState::Loading` state.
// When done loading, the assets will be inserted as resources, and it will transition to the `GameState::MainMenu` state.
impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        AssetLoader::new(GameState::Loading)
            .continue_to_state(GameState::MainMenu)
            .with_collection::<FontAssets>()
            .with_collection::<CharacterTextureAssets>()
            .with_collection::<BlockTextureAssets>()
            .with_collection::<UITextureAssets>()
            .build(app);
    }
    fn name(&self) -> &str {
        "Assets"
    }
}

