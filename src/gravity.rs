use bevy::{core::FixedTimestep, prelude::*};

use crate::pieces::{Piece, PieceBlock, Position};

pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(Self::update_piece_position),
        );
    }
}

impl GravityPlugin {
    pub fn update_piece_position(
        mut pieces: Query<(Entity, &Piece, &mut Position), Without<PieceBlock>>,
    ) {
        for (_e, _piece, mut position) in pieces.iter_mut() {
            // TODO: workout colisions
            if position.1 > 0 {
                position.1 -= 1;
            }
        }
    }
}
