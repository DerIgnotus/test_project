use bevy::prelude::*;

use crate::pieces::components::PieceColor;

pub const TILE_SIZE: f32 = 89.5;
pub const BOARD_OFFSET: Vec2 = Vec2::new(135.0, 135.0);

// Used as an event and tracks the current tile when the mouse is clicked
#[derive(Resource, Reflect, Default, Debug, Clone, Event)]
pub struct CurrentTile(pub (u8, u8));

// basically it needs to have a selected piece and if it has
// it'll check if the second thing selected is a piece or a tile
// it'll do different things depending on what is selected
#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct Selections {
    pub selected_piece: Option<Entity>,

    pub second_selected_piece: Option<Entity>,
    pub second_selected_tile: Option<(u8, u8)>,
}

// Again an event with some data
#[derive(Resource, Reflect, Event)]
pub struct MovePiece {
    pub piece: Entity,
    pub from: (u8, u8),
    pub to: (u8, u8),
}

#[derive(Resource, Reflect, Event)]
pub struct MoveMade();

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct GameState {
    pub turn: PieceColor,
    pub check: bool,
    pub checkmate: bool,
    pub stalemate: bool,
}
