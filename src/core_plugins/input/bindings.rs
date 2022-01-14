use crate::remote::*;
use bevy::prelude::*;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, PartialEq, Clone, Eq, Hash, Serialize, Deserialize, strum_macros::Display)]
pub enum ActionBindingKind {
    Movement(MovementBindingKind),
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Serialize, Deserialize, strum_macros::Display)]
pub enum MovementBindingKind {
    Left,
    Right,
    Up,
    Down,
    Jump,
}

pub static DEFAULT_KEYBINDINGS: Lazy<HashMap<ActionBindingKind, ActionBindingHolder>> =
    Lazy::new(|| {
        let mut m = HashMap::new();
        m.insert(
            ActionBindingKind::Movement(MovementBindingKind::Left),
            ActionBindingHolder::from_single_axis(
                KeyCode::A,
                ControllerAxis(GamepadAxisType::LeftStickX, -1),
            ),
        );
        m.insert(
            ActionBindingKind::Movement(MovementBindingKind::Right),
            ActionBindingHolder::from_single_axis(
                KeyCode::A,
                ControllerAxis(GamepadAxisType::LeftStickX, 1),
            ),
        );
        m.insert(
            ActionBindingKind::Movement(MovementBindingKind::Up),
            ActionBindingHolder::from_single_axis(
                KeyCode::W,
                ControllerAxis(GamepadAxisType::LeftStickY, -1),
            ),
        );
        m.insert(
            ActionBindingKind::Movement(MovementBindingKind::Down),
            ActionBindingHolder::from_single_axis(
                KeyCode::S,
                ControllerAxis(GamepadAxisType::LeftStickY, 1),
            ),
        );
        m.insert(
            ActionBindingKind::Movement(MovementBindingKind::Jump),
            ActionBindingHolder::from_single(KeyCode::Space, GamepadButtonType::South),
        );
        m
    });

#[derive(Debug, PartialEq, Clone, Eq, Hash, Serialize, Deserialize)]
pub struct ActionBindingHolder {
    pub keyboard_combination: KeyboardCombination,
    pub controller_combination: ControllerCombination,
}

impl ActionBindingHolder {
    pub fn from_single(keyboard: KeyCode, controller: GamepadButtonType) -> Self {
        ActionBindingHolder {
            keyboard_combination: KeyboardCombination::SingleKey(keyboard),
            controller_combination: ControllerCombination::SingleButton(controller),
        }
    }
    pub fn from_single_axis(keyboard: KeyCode, controller: ControllerAxis) -> Self {
        ActionBindingHolder {
            keyboard_combination: KeyboardCombination::SingleKey(keyboard),
            controller_combination: ControllerCombination::SingleAxis(controller),
        }
    }
    pub fn new(keyboard: KeyboardCombination, controller: ControllerCombination) -> Self {
        ActionBindingHolder {
            keyboard_combination: keyboard,
            controller_combination: controller,
        }
    }
}

// ------------------------------------------------------------------------------------------------ //
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ControllerAxis(
    #[serde(with = "GamepadAxisTypeDefinition")] pub GamepadAxisType,
    pub i8,
);

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum ControllerCombination {
    SingleAxis(ControllerAxis),
    Axis(ControllerAxis, ControllerAxis),
    SingleButton(#[serde(with = "GamepadButtonTypeDefinition")] GamepadButtonType),
    Button(
        #[serde(with = "GamepadButtonTypeDefinition")] GamepadButtonType,
        #[serde(with = "GamepadButtonTypeDefinition")] GamepadButtonType,
    ),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum KeyboardCombination {
    SingleKey(#[serde(with = "KeyCodeDefinition")] KeyCode),
    Keys(
        #[serde(with = "KeyCodeDefinition")] KeyCode,
        #[serde(with = "KeyCodeDefinition")] KeyCode,
    ),
    SingleMouseButton(#[serde(with = "MouseButtonDefinition")] MouseButton),
    MouseButtons(
        #[serde(with = "MouseButtonDefinition")] MouseButton,
        #[serde(with = "MouseButtonDefinition")] MouseButton,
    ),
    MouseAndKeyboard(
        #[serde(with = "KeyCodeDefinition")] KeyCode,
        #[serde(with = "MouseButtonDefinition")] MouseButton,
    ),
}
