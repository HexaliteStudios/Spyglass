use bevy::prelude::Msaa;

pub fn build_anti_aliasing() -> Msaa {
    Msaa { samples: 4 }
}