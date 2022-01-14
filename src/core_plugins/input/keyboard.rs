use crate::prelude::*;
use bevy::input::ElementState;
use bevy::prelude::*;

pub struct KeyboardSupportPlugin;

// Plugin responsible for sending keyboard input events to the main input system.
// This plugin is only active in the `GameState::Playing` state.

impl Plugin for KeyboardSupportPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(listen_to_keyboard_input);
    }
}

fn listen_to_keyboard_input(
    mut krd: Res<Input<KeyCode>>,
    mut mrd: Res<Input<MouseButton>>,
    mut wrt: EventWriter<InputEvent>,
    mut conf: Res<SpyglassSettings>,
) {
    for (kind, bind) in conf.controls.keybindings.iter() {
        let element = match &bind.keyboard_combination {
            KeyboardCombination::SingleKey(code) => get_element_state(code.clone(), &krd),
            KeyboardCombination::Keys(c1, c2) => check_combination(c1.clone(), c2.clone(), &krd),
            KeyboardCombination::SingleMouseButton(bt) => get_element_state(bt.clone(), &mrd),
            KeyboardCombination::MouseButtons(m1, m2) => {
                check_combination(m1.clone(), m2.clone(), &mrd)
            }
            KeyboardCombination::MouseAndKeyboard(c1, m2) => {
                let c1_result = get_element_state(c1.clone(), &krd);
                if c1_result == get_element_state(m2.clone(), &mrd) {
                    c1_result
                } else {
                    None
                }
            }
        };
        if let Some(e) = element {
            wrt.send(InputEvent::new(kind.clone(), e, InputKind::Keyboard));
        }
    }
}
