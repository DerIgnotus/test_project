use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use components::*;
use systems::*;

pub struct PiecesPlugin;

// Registering the components is important so they show up in the inspector

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ChessPiece>()
            .register_type::<PieceType>()
            .register_type::<PieceColor>()
            .add_systems(Startup, set_up_game);
    }
}
