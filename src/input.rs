use bevy::prelude::*;

use crate::pieces::{Direction, RotationEvent};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::handle_keyboard_input);
    }
}

impl InputPlugin {
    pub fn handle_keyboard_input(keys: Res<Input<KeyCode>>, mut rot: EventWriter<RotationEvent>) {
        if keys.just_pressed(KeyCode::Z) {
            rot.send(RotationEvent {
                direction: Direction::CounterClockwise,
            });
        }
        if keys.just_pressed(KeyCode::X) {
            rot.send(RotationEvent {
                direction: Direction::Clockwise,
            });
        }
    }
}
