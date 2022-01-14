use bevy::prelude::*;

#[derive(Component)]
pub struct Health(pub u8);

impl Health {
    pub fn new(health: u8) -> Self {
        Health(health)
    }

    pub fn maximum() -> u8 { 3 }

    pub fn alive(&self) -> bool {
        self.0 > 0
    }

    fn add(&mut self) {
        if self.0 < Health::maximum() {
            self.0 += 1;
        }
    }

    pub fn subtract(&mut self) {
        if self.0 > 0 {
            self.0 -= 1;
        }
    }
}