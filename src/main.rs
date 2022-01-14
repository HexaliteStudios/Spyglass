use bevy::{app::App, DefaultPlugins, prelude::*};
use spyglass::AsyncResult;
use spyglass::plugins::GamePlugin;
use spyglass::resources::build_window_descriptor;
use spyglass::state::GameState;

#[tokio::main]
async fn main() -> AsyncResult<()> {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(build_window_descriptor())
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
    Ok(())
}