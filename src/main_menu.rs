use bevy::prelude::*;
use crate::prelude::*;

pub struct MainMenuPlugin;

// This plugin is responsible for adding a main menu to the game, that contains some buttons.
// This menu is only drawn when the game is in the `GameState::MainMenu` state.

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(menu));
    }
}

fn menu(mut commands: Commands, font_assets: Res<FontAssets>, mut game_state: ResMut<State<GameState>>) {
    //commands.spawn_bundle(UiCameraBundle::default());
    //commands.spawn_bundle(build_text(font_assets, "Hello, world!", None, None, build_style(5., 5.)));
    game_state.set(GameState::Playing).unwrap();
}
