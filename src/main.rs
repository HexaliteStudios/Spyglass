use bevy::prelude::*;
use spyglass::prelude::*;

pub struct GamePlugin;

// This is the main plugin, where it load all necessary components and menu to run our
// application.
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_startup_system(init_data_system)
            .add_plugin(AssetsPlugin)
            .add_plugin(MainMenuPlugin)
            .add_plugin(InputPlugin);
    }
}

#[tokio::main]
async fn main() -> AsyncResult<()> {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(build_anti_aliasing())
        .insert_resource(build_window_descriptor())
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
    Ok(())
}
