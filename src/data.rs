use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::core_plugins::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct SpyglassSettings {
    pub video: SpyglassVideoSettings,
    pub controls: SpyglassControlsSettings,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpyglassControlSettings {
    pub keybindings: HashMap<ActionBindingKind, ActionBindingHolder>,
}

impl Default for SpyglassControlSettings {
    fn default() -> Self {
        Self {
            keybindings: *DEFAULT_KEYBINDINGS,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpyglassVideoSettings {
    #[derivative(Default(value = "-1"))]
    pub window_size: (u32, u32),
    pub is_fullscreen: bool,
}

pub fn init_data_system() {

}