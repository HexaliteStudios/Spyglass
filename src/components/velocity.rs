use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Component)]
pub struct Velocity(pub Vec2);

impl From<Vec2> for Velocity {
    fn from(x: Vec2) -> Velocity {
        Velocity(x)
    }
}

impl Into<Vec2> for Velocity {
    fn into(self) -> Vec2 {
        self.0
    }
}
