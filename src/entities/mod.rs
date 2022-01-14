// A Entity may also be considered a "component", but its most likely a "separator tag", where
// its purpose is to distinguish between different types of entities.

mod player;
pub use player::*;

mod entity;
pub use entity::*;