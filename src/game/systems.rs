use bevy::prelude::*;

use crate::{
    game::resources::*,
    pieces::components::{ChessPiece, PieceColor, PieceType},
};

pub fn mouse_input(
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut current_tile: EventWriter<CurrentTile>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = q_camera.single().unwrap();
        let window = q_window.single().unwrap();

        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
            .map(|ray| ray.origin.truncate())
        {
            let tile_coord = screen_coord_to_tile((world_position.x, world_position.y));

            if (tile_coord.0, tile_coord.1) == (0, 0) {
                println!("Clicked outside the board.");
                return;
            }

            // Calls the event with the data (write = call event. current_tile is an event)
            current_tile.write(CurrentTile((tile_coord.0, tile_coord.1)));
        }
    }
}

pub fn tile_clicked(
    mut events: EventReader<CurrentTile>,
    mut selections: ResMut<Selections>,
    query: Query<(Entity, &ChessPiece)>,
    mut move_piece_events: EventWriter<MovePiece>,
) {
    for CurrentTile(tile) in events.read() {
        println!("Tile clicked: {:?}", tile);

        // if no first piece is selected
        if selections.selected_piece.is_none() && selections.second_selected_piece.is_none() {
            if let Some((entity, _piece)) = query.iter().find(|(_, piece)| piece.position == *tile)
            {
                println!("Selected piece: {:?}", entity);
                selections.selected_piece = Some(entity);
            } else {
                println!("No piece at tile: {:?}", tile);
            }
        }
        // tile gets selected
        else if let Some(selected_entity) = selections.selected_piece {
            if let Ok(piece) = query.get(selected_entity) {
                // If the clicked tile has a piece of the same color, change selection
                if let Some((entity, clicked_piece)) =
                    query.iter().find(|(_, p)| p.position == *tile)
                {
                    if clicked_piece.color == piece.1.color {
                        selections.selected_piece = Some(entity);
                        selections.second_selected_piece = None;
                        selections.second_selected_tile = None;
                        continue;
                    }
                }
                // Otherwise, attempt to move
                move_piece_events.write(MovePiece {
                    piece: selected_entity,
                    from: piece.1.position,
                    to: *tile,
                });
            }
        }
    }
}

pub fn move_piece(
    mut events: EventReader<MovePiece>,
    mut selections: ResMut<Selections>,
    mut query_set: ParamSet<(
        Query<&mut ChessPiece>,
        Query<(Entity, &ChessPiece, &mut Transform)>,
    )>,
) {
    for MovePiece { piece, from, to } in events.read() {
        // checks whether the second selection is a piece or a tile
        // second selection is a piece
        if let Some((other_entity, _, _)) =
            query_set.p1().iter().find(|(_, p, _)| p.position == *to)
        {
            selections.second_selected_piece = Some(other_entity);
        }
        // second selection is a tile
        else {
            selections.second_selected_tile = Some(*to);

            // Collect the pieces first to avoid multiple mutable borrows
            let pieces: Vec<ChessPiece> = query_set
                .p1()
                .iter()
                .map(|(_, p, _)| p.clone())
                .collect::<Vec<ChessPiece>>();

            // Use the first query (mutable) to try moving the piece
            if let Ok(mut moving_piece) = query_set.p0().get_mut(*piece) {
                if can_move_to_tile(&moving_piece, *to, &pieces.iter().collect::<Vec<_>>()) {
                    println!("Moving piece {:?} from {:?} to {:?}", piece, from, to);
                    moving_piece.position = *to;

                    // Also update the transform position
                    if let Ok((_, _, mut transform)) = query_set.p1().get_mut(*piece) {
                        let (x, y) = tile_to_screen_coord(*to);
                        transform.translation.x = x;
                        transform.translation.y = y;
                    }

                    selections.selected_piece = None;
                    selections.second_selected_piece = None;
                    selections.second_selected_tile = None;
                } else {
                    selections.second_selected_piece = None;
                    selections.second_selected_tile = None;

                    println!("Invalid move for piece: {:?}", moving_piece);
                }
            }
        }
    }
}

fn can_move_to_tile(piece: &ChessPiece, to_tile: (u8, u8), pieces: &Vec<&ChessPiece>) -> bool {
    // Helper to check if a tile is occupied by any piece
    let is_occupied = |pos: (u8, u8)| pieces.iter().any(|p| p.position == pos);

    match piece.piece {
        PieceType::Pawn => {
            let dy = to_tile.1 as i8 - piece.position.1 as i8;
            let dx = (to_tile.0 as i8 - piece.position.0 as i8).abs();

            match piece.color {
                PieceColor::White => {
                    // White pawns move up (increasing y)
                    if piece.position.1 == 2 && dy == 2 && dx == 0 {
                        // First move, two squares forward
                        let intermediate = (piece.position.0, piece.position.1 + 1);
                        if !is_occupied(intermediate) && !is_occupied(to_tile) {
                            return true;
                        }
                    } else if dy == 1 && dx == 0 {
                        // Normal move, one square forward
                        if !is_occupied(to_tile) {
                            return true;
                        }
                    } else if dy == 1 && dx == 1 {
                        // Capture diagonally
                        if let Some(target) = pieces.iter().find(|p| p.position == to_tile) {
                            if target.color != piece.color {
                                return true;
                            }
                        }
                    }
                }
                PieceColor::Black => {
                    // Black pawns move down (decreasing y)
                    if piece.position.1 == 7 && dy == -2 && dx == 0 {
                        // First move, two squares forward
                        let intermediate = (piece.position.0, piece.position.1 - 1);
                        if !is_occupied(intermediate) && !is_occupied(to_tile) {
                            return true;
                        }
                    } else if dy == -1 && dx == 0 {
                        // Normal move, one square forward
                        if !is_occupied(to_tile) {
                            return true;
                        }
                    } else if dy == -1 && dx == 1 {
                        // Capture diagonally
                        if let Some(target) = pieces.iter().find(|p| p.position == to_tile) {
                            if target.color != piece.color {
                                return true;
                            }
                        }
                    }
                }
            }
            false
        }
        PieceType::Rook => {
            // Rooks move in straight lines
            if piece.position.0 == to_tile.0 {
                // Vertical move
                let range = if piece.position.1 < to_tile.1 {
                    (piece.position.1 + 1)..to_tile.1
                } else {
                    (to_tile.1 + 1)..piece.position.1
                };
                for y in range {
                    if is_occupied((piece.position.0, y)) {
                        return false;
                    }
                }
                // Can't capture own piece
                if let Some(target) = pieces.iter().find(|p| p.position == to_tile) {
                    return target.color != piece.color;
                }
                true
            } else if piece.position.1 == to_tile.1 {
                // Horizontal move
                let range = if piece.position.0 < to_tile.0 {
                    (piece.position.0 + 1)..to_tile.0
                } else {
                    (to_tile.0 + 1)..piece.position.0
                };
                for x in range {
                    if is_occupied((x, piece.position.1)) {
                        return false;
                    }
                }
                if let Some(target) = pieces.iter().find(|p| p.position == to_tile) {
                    return target.color != piece.color;
                }
                true
            } else {
                false
            }
        }
        PieceType::Bishop => {
            // Bishops move diagonally
            let dx = to_tile.0 as i8 - piece.position.0 as i8;
            let dy = to_tile.1 as i8 - piece.position.1 as i8;
            if dx.abs() == dy.abs() {
                let steps = dx.abs();
                let x_step = if dx > 0 { 1 } else { -1 };
                let y_step = if dy > 0 { 1 } else { -1 };
                for i in 1..steps {
                    let x = (piece.position.0 as i8 + i * x_step) as u8;
                    let y = (piece.position.1 as i8 + i * y_step) as u8;
                    if is_occupied((x, y)) {
                        return false;
                    }
                }
                if let Some(target) = pieces.iter().find(|p| p.position == to_tile) {
                    return target.color != piece.color;
                }
                true
            } else {
                false
            }
        }
        PieceType::Queen => {
            // Queens move like rook or bishop
            let dx = to_tile.0 as i8 - piece.position.0 as i8;
            let dy = to_tile.1 as i8 - piece.position.1 as i8;
            if dx.abs() == dy.abs() {
                // Diagonal like bishop
                let steps = dx.abs();
                let x_step = if dx > 0 { 1 } else { -1 };
                let y_step = if dy > 0 { 1 } else { -1 };
                for i in 1..steps {
                    let x = (piece.position.0 as i8 + i * x_step) as u8;
                    let y = (piece.position.1 as i8 + i * y_step) as u8;
                    if is_occupied((x, y)) {
                        return false;
                    }
                }
                if let Some(target) = pieces.iter().find(|p| p.position == to_tile) {
                    return target.color != piece.color;
                }
                true
            } else if piece.position.0 == to_tile.0 {
                // Vertical like rook
                let range = if piece.position.1 < to_tile.1 {
                    (piece.position.1 + 1)..to_tile.1
                } else {
                    (to_tile.1 + 1)..piece.position.1
                };
                for y in range {
                    if is_occupied((piece.position.0, y)) {
                        return false;
                    }
                }
                if let Some(target) = pieces.iter().find(|p| p.position == to_tile) {
                    return target.color != piece.color;
                }
                true
            } else if piece.position.1 == to_tile.1 {
                // Horizontal like rook
                let range = if piece.position.0 < to_tile.0 {
                    (piece.position.0 + 1)..to_tile.0
                } else {
                    (to_tile.0 + 1)..piece.position.0
                };
                for x in range {
                    if is_occupied((x, piece.position.1)) {
                        return false;
                    }
                }
                if let Some(target) = pieces.iter().find(|p| p.position == to_tile) {
                    return target.color != piece.color;
                }
                true
            } else {
                false
            }
        }
        PieceType::Knight => {
            // Knights move in L-shape and can jump over pieces
            let dx = (to_tile.0 as i8 - piece.position.0 as i8).abs();
            let dy = (to_tile.1 as i8 - piece.position.1 as i8).abs();
            if (dx == 2 && dy == 1) || (dx == 1 && dy == 2) {
                if let Some(target) = pieces.iter().find(|p| p.position == to_tile) {
                    return target.color != piece.color;
                }
                true
            } else {
                false
            }
        }
        PieceType::King => {
            // Kings move one square in any direction
            let dx = (to_tile.0 as i8 - piece.position.0 as i8).abs();
            let dy = (to_tile.1 as i8 - piece.position.1 as i8).abs();
            if dx <= 1 && dy <= 1 {
                if let Some(target) = pieces.iter().find(|p| p.position == to_tile) {
                    return target.color != piece.color;
                }
                true
            } else {
                false
            }
        }
    }
}

// I think I needed this once but since I now query the pieces directly
// I don't need this anymore (I think)
// Still keeping it here cause yk
/*
fn get_piece_entity_at_tile<'a>(
    tile: &(u8, u8),
    query: &'a Query<(Entity, &'a ChessPiece)>,
) -> Option<(Entity, &'a ChessPiece)> {
    query.iter().find(|(_, piece)| piece.position == *tile)
}
*/

// well so I can get the selected tile not just the screen coordinates
fn screen_coord_to_tile(screen_coord: (f32, f32)) -> (u8, u8) {
    let tile_pos = (
        // divided by 1.5 cause the offset was too big but only for this otherwise it's fine
        ((screen_coord.0 - BOARD_OFFSET.x / 1.5) / TILE_SIZE).floor() as u8 + 1,
        ((screen_coord.1 - BOARD_OFFSET.y / 1.5) / TILE_SIZE).floor() as u8 + 1,
    );

    if (screen_coord.0 - BOARD_OFFSET.x / 1.5) < 0.0
        || (screen_coord.1 - BOARD_OFFSET.y / 1.5) < 0.0
        || tile_pos.0 > 8
        || tile_pos.1 > 8
    {
        return (0, 0);
    }

    // this does not need to be called but I want to see the output
    tile_to_screen_coord(tile_pos);

    tile_pos
}

// I don't think I actually need this function
fn tile_to_screen_coord(tile: (u8, u8)) -> (f32, f32) {
    let screen_coord = (
        BOARD_OFFSET.x + (tile.0 as f32 - 1.0) * TILE_SIZE,
        BOARD_OFFSET.y + (tile.1 as f32 - 1.0) * TILE_SIZE,
    );

    println!(
        "Tile coords: {:?} -> Screen coords: {:?}",
        tile, screen_coord
    );

    screen_coord
}
