use bevy::prelude::*;

use crate::pieces::components::*;

use crate::game::resources::BOARD_OFFSET;
use crate::game::resources::TILE_SIZE;

pub fn set_up_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite::from_image(asset_server.load("images/board.png")),
        Transform::from_xyz(450.0, 450.0, -0.1),
        GlobalTransform::default(),
    ));

    // 6 types of pieces

    for i in 0..6 {
        match i {
            0 => spawn_pawns(&mut commands, &asset_server),
            1 => spawn_knights(&mut commands, &asset_server),
            2 => spawn_rooks(&mut commands, &asset_server),
            3 => spawn_bishops(&mut commands, &asset_server),
            4 => spawn_queens(&mut commands, &asset_server),
            5 => spawn_kings(&mut commands, &asset_server),
            _ => println!("{} idk too much of a number.", i),
        }
    }
}

fn spawn_pawns(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    // Could do a nested loop for both colors
    // but this is easier to read and understand

    for white_pawn in 1..9 {
        // Start at 1 or else it would multiply by 0
        // Also for the name so the names start at 1 not 0

        let pos = Vec3::new(
            BOARD_OFFSET.x + (white_pawn as f32 - 1.0) * TILE_SIZE,
            BOARD_OFFSET.y + (2 as f32 - 1.0) * TILE_SIZE,
            0.0,
        );

        commands.spawn((
            Sprite::from_image(asset_server.load("images/w_Pawn.png")),
            Transform {
                translation: pos,
                scale: Vec3::splat(0.7),
                ..Default::default()
            },
            ChessPiece::new(
                &format!("Pawn {}", white_pawn),
                PieceType::Pawn,
                PieceColor::White,
                (white_pawn, 2),
                1,
            ),
        ));
    }

    for black_pawn in 1..9 {
        let pos = Vec3::new(
            BOARD_OFFSET.x + (black_pawn as f32 - 1.0) * TILE_SIZE,
            BOARD_OFFSET.y + (7 as f32 - 1.0) * TILE_SIZE,
            0.0,
        );

        commands.spawn((
            Sprite::from_image(asset_server.load("images/b_Pawn.png")),
            Transform {
                translation: pos,
                scale: Vec3::splat(0.7),
                ..Default::default()
            },
            ChessPiece::new(
                &format!("Pawn {}", black_pawn),
                PieceType::Pawn,
                PieceColor::Black,
                (black_pawn, 7),
                1,
            ),
        ));
    }
}

fn spawn_knights(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    // See "spawn_pawns" for explanation

    for white_knight in 1..3 {
        let pos_x: u8 = if white_knight == 1 { 2 } else { 7 };

        let pos = Vec3::new(
            BOARD_OFFSET.x + (pos_x as f32 - 1.0) * TILE_SIZE,
            BOARD_OFFSET.y + (1 as f32 - 1.0) * TILE_SIZE,
            0.0,
        );

        commands.spawn((
            Sprite::from_image(asset_server.load("images/w_Knight.png")),
            Transform {
                translation: pos,
                scale: Vec3::splat(0.7),
                ..Default::default()
            },
            ChessPiece::new(
                &format!("Knight {}", white_knight),
                PieceType::Knight,
                PieceColor::White,
                (pos_x, 1),
                3,
            ),
        ));
    }

    for black_knight in 1..3 {
        let pos_x: u8 = if black_knight == 1 { 2 } else { 7 };

        let pos = Vec3::new(
            BOARD_OFFSET.x + (pos_x as f32 - 1.0) * TILE_SIZE,
            BOARD_OFFSET.y + (8 as f32 - 1.0) * TILE_SIZE,
            0.0,
        );

        commands.spawn((
            Sprite::from_image(asset_server.load("images/b_Knight.png")),
            Transform {
                translation: pos,
                scale: Vec3::splat(0.7),
                ..Default::default()
            },
            ChessPiece::new(
                &format!("Knight {}", black_knight),
                PieceType::Knight,
                PieceColor::Black,
                (pos_x, 8),
                3,
            ),
        ));
    }
}

fn spawn_rooks(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    // See "spawn_pawns" for explanation

    for white_rook in 1..3 {
        let pos_x: u8 = if white_rook == 1 { 1 } else { 8 };

        let pos = Vec3::new(
            BOARD_OFFSET.x + (pos_x as f32 - 1.0) * TILE_SIZE,
            BOARD_OFFSET.y + (1 as f32 - 1.0) * TILE_SIZE,
            0.0,
        );

        commands.spawn((
            Sprite::from_image(asset_server.load("images/w_Rook.png")),
            Transform {
                translation: pos,
                scale: Vec3::splat(0.7),
                ..Default::default()
            },
            ChessPiece::new(
                &format!("Rook {}", white_rook),
                PieceType::Rook,
                PieceColor::White,
                (pos_x, 1),
                5,
            ),
        ));
    }

    for black_rook in 1..3 {
        let pos_x: u8 = if black_rook == 1 { 1 } else { 8 };

        let pos = Vec3::new(
            BOARD_OFFSET.x + (pos_x as f32 - 1.0) * TILE_SIZE,
            BOARD_OFFSET.y + (8 as f32 - 1.0) * TILE_SIZE,
            0.0,
        );

        commands.spawn((
            Sprite::from_image(asset_server.load("images/b_Rook.png")),
            Transform {
                translation: pos,
                scale: Vec3::splat(0.7),
                ..Default::default()
            },
            ChessPiece::new(
                &format!("Rook {}", black_rook),
                PieceType::Rook,
                PieceColor::Black,
                (pos_x, 8),
                5,
            ),
        ));
    }
}

fn spawn_bishops(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    // See "spawn_pawns" for explanation

    for white_bishop in 1..3 {
        let pos_x: u8 = if white_bishop == 1 { 3 } else { 6 };

        let pos = Vec3::new(
            BOARD_OFFSET.x + (pos_x as f32 - 1.0) * TILE_SIZE,
            BOARD_OFFSET.y + (1 as f32 - 1.0) * TILE_SIZE,
            0.0,
        );

        commands.spawn((
            Sprite::from_image(asset_server.load("images/w_Bishop.png")),
            Transform {
                translation: pos,
                scale: Vec3::splat(0.7),
                ..Default::default()
            },
            ChessPiece::new(
                &format!("Bishop {}", white_bishop),
                PieceType::Bishop,
                PieceColor::White,
                (pos_x, 1),
                3,
            ),
        ));
    }

    for black_bishop in 1..3 {
        let pos_x: u8 = if black_bishop == 1 { 3 } else { 6 };

        let pos = Vec3::new(
            BOARD_OFFSET.x + (pos_x as f32 - 1.0) * TILE_SIZE,
            BOARD_OFFSET.y + (8 as f32 - 1.0) * TILE_SIZE,
            0.0,
        );

        commands.spawn((
            Sprite::from_image(asset_server.load("images/b_Bishop.png")),
            Transform {
                translation: pos,
                scale: Vec3::splat(0.7),
                ..Default::default()
            },
            ChessPiece::new(
                &format!("Bishop {}", black_bishop),
                PieceType::Bishop,
                PieceColor::Black,
                (pos_x, 8),
                3,
            ),
        ));
    }
}

fn spawn_queens(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let pos = Vec3::new(
        BOARD_OFFSET.x + (4 as f32 - 1.0) * TILE_SIZE,
        BOARD_OFFSET.y + (1 as f32 - 1.0) * TILE_SIZE,
        0.0,
    );

    commands.spawn((
        Sprite::from_image(asset_server.load("images/w_Queen.png")),
        Transform {
            translation: pos,
            scale: Vec3::splat(0.7),
            ..Default::default()
        },
        ChessPiece::new("Queen", PieceType::Queen, PieceColor::White, (4, 1), 9),
    ));

    let pos = Vec3::new(
        BOARD_OFFSET.x + (4 as f32 - 1.0) * TILE_SIZE,
        BOARD_OFFSET.y + (8 as f32 - 1.0) * TILE_SIZE,
        0.0,
    );

    commands.spawn((
        Sprite::from_image(asset_server.load("images/b_Queen.png")),
        Transform {
            translation: pos,
            scale: Vec3::splat(0.7),
            ..Default::default()
        },
        ChessPiece::new("Queen", PieceType::Queen, PieceColor::Black, (4, 8), 9),
    ));
}

fn spawn_kings(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    // Value of the king is 0 since it cannot be captured

    let pos = Vec3::new(
        BOARD_OFFSET.x + (5 as f32 - 1.0) * TILE_SIZE,
        BOARD_OFFSET.y + (1 as f32 - 1.0) * TILE_SIZE,
        0.0,
    );

    commands.spawn((
        Sprite::from_image(asset_server.load("images/w_King.png")),
        Transform {
            translation: pos,
            scale: Vec3::splat(0.7),
            ..Default::default()
        },
        ChessPiece::new("King", PieceType::King, PieceColor::White, (5, 1), 0),
    ));

    let pos = Vec3::new(
        BOARD_OFFSET.x + (5 as f32 - 1.0) * TILE_SIZE,
        BOARD_OFFSET.y + (8 as f32 - 1.0) * TILE_SIZE,
        0.0,
    );

    commands.spawn((
        Sprite::from_image(asset_server.load("images/b_King.png")),
        Transform {
            translation: pos,
            scale: Vec3::splat(0.7),
            ..Default::default()
        },
        ChessPiece::new("King", PieceType::King, PieceColor::Black, (5, 8), 0),
    ));
}
