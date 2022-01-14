use crate::prelude::*;
use bevy::input::ElementState;
use bevy::prelude::*;

struct SpyglassGamepad(Gamepad);
// ----------------------------------------------------------------------------- //

pub struct ControllerSupportPlugin;

// Plugin responsible for sending controller input events to the main input system.
// This plugin is only active in the `GameState::Playing` state.

impl Plugin for ControllerSupportPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(listen_to_controller_connection)
            .add_system(listen_to_controller_input);
    }
}

fn listen_to_controller_connection(
    mut commands: Commands,
    gamepad: Option<Res<SpyglassGamepad>>,
    mut rd: EventReader<GamepadEvent>,
) {
    for GamepadEvent(id, kind) in rd.iter() {
        match kind {
            GamepadEventType::Connected => {
                // if we don't have any gamepad yet, use this one
                if gamepad.is_none() {
                    commands.insert_resource(SpyglassGamepad(*id));
                }
            }
            GamepadEventType::Disconnected => {
                if let Some(SpyglassGamepad(old_id)) = gamepad.as_deref() {
                    if old_id == id {
                        commands.remove_resource::<SpyglassGamepad>();
                    }
                }
            }
            _ => {}
        }
    }
}

fn listen_to_controller_input(
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,
    gamepad: Option<Res<SpyglassGamepad>>,
    mut conf: Res<SpyglassSettings>,
    mut wrt: EventWriter<InputEvent>,
) {
    let gamepad = if let Some(gamepad) = gamepad.as_deref() {
        gamepad.0
    } else {
        return;
    };

    let axis_lx = GamepadAxis(gamepad, GamepadAxisType::LeftStickX);
    let axis_ly = GamepadAxis(gamepad, GamepadAxisType::LeftStickY);

    for (kind, bind) in conf.controls.keybindings.iter() {
        let element: Option<ElementState> = match &bind.controller_combination {
            ControllerCombination::SingleAxis(axis) => {
                let gamepad_axis = axes.get(if axis.0 == GamepadAxisType::LeftStickX {
                    axis_lx
                } else {
                    axis_ly
                });
                if let Some(gamepad_axis) = gamepad_axis {
                    if gamepad_axis.abs() - conf.controls.controller_dead_zone
                        >= axis.1.abs() as f32
                    {
                        Some(ElementState::Pressed)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            ControllerCombination::Axis(axis1, axis2) => {
                let x = axes.get(axis_lx);
                let y = axes.get(axis_ly);
                if let (Some(x), Some(y)) = (x, y) {
                    let x = x - conf.controls.controller_dead_zone;
                    let y = y - conf.controls.controller_dead_zone;
                    if x >= axis1.1 as f32 && y >= axis2.1 as f32 {
                        Some(ElementState::Pressed)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            ControllerCombination::SingleButton(button) => {
                get_element_state(GamepadButton(gamepad, button.clone()), &buttons)
            }
            ControllerCombination::Button(button1, button2) => check_combination(
                GamepadButton(gamepad, button1.clone()),
                GamepadButton(gamepad, button2.clone()),
                &buttons,
            ),
            _ => None,
        };
        if let Some(e) = element {
            wrt.send(InputEvent::new(kind.clone(), e, InputKind::Controller));
        }
    }
}
