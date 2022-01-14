use bevy::prelude::*;

struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(velocity_system);
    }
}

fn velocity_system() {

}
