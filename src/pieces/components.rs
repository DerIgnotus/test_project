use bevy::prelude::*;
use std::fmt;

// reflect stuff so it shows up in the inspector

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct ChessPiece {
    name: String,
    pub piece: PieceType,
    pub color: PieceColor,
    pub position: (u8, u8),
    value: u8,
}

#[derive(Reflect, Clone, Debug, PartialEq, Eq, Default)]
pub enum PieceType {
    #[default]
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Reflect, Clone, Debug, PartialEq, Eq, Default)]
pub enum PieceColor {
    #[default]
    White,
    Black,
}

// Nicer to create a new ChessPiece with this function
// instead of manually setting all fields
impl ChessPiece {
    pub fn new(
        name: &str,
        piece: PieceType,
        color: PieceColor,
        position: (u8, u8),
        value: u8,
    ) -> ChessPiece {
        ChessPiece {
            name: name.to_string(),
            piece,
            color,
            position,
            value,
        }
    }
}

// Better display for print statements
impl fmt::Display for ChessPiece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} {:?} at {:?}        -        {} {}",
            self.color, self.piece, self.position, self.name, self.value,
        )
    }
}
