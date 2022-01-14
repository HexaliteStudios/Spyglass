pub mod assets;
pub mod components;
pub mod core_plugins;
pub mod data;
pub mod entities;
pub mod events;
pub mod main_menu;
pub mod prelude;
pub mod remote;
pub mod resources;
pub mod state;

pub type AsyncResult<T> = Result<T, Box<dyn std::error::Error>>;
