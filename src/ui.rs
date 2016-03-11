use super::*;
use trident::*;
use lux::prelude::*;
use lux::interactive::keycodes::VirtualKeyCode;
use std::collections::HashMap;

pub type KeyBindings = HashMap<Action, Vec<VirtualKeyCode>>;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Action {
    CameraUp,
    CameraDown,
    CameraLeft,
    CameraRight,
    CameraZoomModifier,
}

pub fn update_camera(position: &mut (f32, f32), scale: &mut f32, window: &Window, bindings: &KeyBindings) {
    fn is_any_down(action: Action, bindings: &KeyBindings, window: &Window) -> bool {
        if let Some(keys) = bindings.get(&action) {
            for &key in keys {
                if window.is_key_pressed(key) {
                    return true;
                }
            }
            false
        } else {
            panic!("no keybindings for {:?}", action);
        }
    }
    let mut did_scale = false;

    if is_any_down(Action::CameraZoomModifier, bindings, window) && is_any_down(Action::CameraUp, bindings, window) {
        *scale *= 1.0 + 0.01;
        did_scale = true;
    }

    if is_any_down(Action::CameraZoomModifier, bindings, window) && is_any_down(Action::CameraDown, bindings, window) {
        *scale *= 1.0 - 0.01;
        did_scale = true;
    }

    if !did_scale && is_any_down(Action::CameraUp, bindings, window) {
        position.1 -= 15.0 / *scale;
    }

    if !did_scale && is_any_down(Action::CameraDown, bindings, window) {
        position.1 += 15.0 / *scale;
    }

    if !did_scale && is_any_down(Action::CameraRight, bindings, window) {
        position.0 += 15.0 / *scale;
    }

    if !did_scale && is_any_down(Action::CameraLeft, bindings, window) {
        position.0 -= 15.0 / *scale;
    }
}
