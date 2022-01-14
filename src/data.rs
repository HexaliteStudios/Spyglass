use crate::core_plugins::*;
use bevy::prelude::Commands;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::{create_dir_all, File};
use std::io::{Error, Read, Write};
use std::iter::Map;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub struct SpyglassSettings {
    pub video: SpyglassVideoSettings,
    pub controls: SpyglassControlSettings,
}

impl Default for SpyglassSettings {
    fn default() -> Self {
        SpyglassSettings {
            video: SpyglassVideoSettings::default(),
            controls: SpyglassControlSettings::default(),
        }
    }
}

#[allow(unused_must_use)]
impl SpyglassSettings {
    fn write(&self) {
        let path = get_settings_path();
        create_dir_all(path.parent().expect("Failed to get the settings file parent."), );
        let mut file = File::create(path).expect("Failed to create the settings file.");
        let to_ron = ron::to_string(self).expect("Failed to serialize settings.");
        file.write_all(to_ron.as_bytes()).expect("Failed to write settings to file.");
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpyglassControlSettings {
    pub keybindings: HashMap<ActionBindingKind, ActionBindingHolder>,
    pub controller_dead_zone: f32
}

impl Default for SpyglassControlSettings {
    fn default() -> Self {
        Self {
            keybindings: DEFAULT_KEYBINDINGS.clone(),
            controller_dead_zone: 0.25
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpyglassVideoSettings {
    pub window_size: (u32, u32),
    pub is_fullscreen: bool,
}

impl Default for SpyglassVideoSettings {
    fn default() -> Self {
        Self {
            window_size: (1280, 720),
            is_fullscreen: false,
        }
    }
}

fn get_settings_path() -> PathBuf {
    let exe = env::current_exe().expect("Failed to get current executable path.");
    let dir = exe
        .parent()
        .expect("The current executable's parent directory should exist.");
    dir.join("data/settings.ron")
}

pub fn init_data_system(mut commands: Commands) {
    let path = get_settings_path();
    let parent = path.parent().expect("Failed to get the settings file parent.");
    create_dir_all(parent);

    let settings: SpyglassSettings = if let Ok(mut f) = File::open(path) {
        let mut buf = String::new();
        f.read_to_string(&mut buf);
        ron::from_str(buf.as_str()).expect("Failed to deserialize settings file.")
    } else {
        let s = SpyglassSettings::default();
        s.write();
        s
    };

    settings.write();
    commands.insert_resource(settings);
}
