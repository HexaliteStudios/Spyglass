use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct CollisionEvent(pub Entity, pub Entity);