use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::pieces::components::PieceColor;

pub struct GamePlugin;

// Important to insert and add everything
// Also no idea why the inser_resource(Selections) doesn't have
// any color highlighting inside the brackets

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Selections>()
            .register_type::<GameState>()
            .insert_resource(CurrentTile((0, 0)))
            .insert_resource(Selections {
                selected_piece: None,
                second_selected_piece: None,
                second_selected_tile: None,
            })
            .insert_resource(GameState {
                turn: PieceColor::White,
                check: false,
                checkmate: false,
                stalemate: false,
            })
            .add_event::<CurrentTile>()
            .add_event::<MovePiece>()
            .add_event::<MoveMade>()
            .add_systems(
                Update,
                (mouse_input, tile_clicked, move_piece, update_ui, move_made),
            );
    }
}
