use bevy::prelude::*;

use crate::{game::resources::*, pieces::components::ChessPiece, pieces::components::PieceType};

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
    mut query_set: ParamSet<(Query<&mut ChessPiece>, Query<(Entity, &ChessPiece)>)>,
) {
    for MovePiece { piece, from, to } in events.read() {
        // checks whether the second selection is a piece or a tile
        // second selection is a piece
        if let Some((other_entity, _)) = query_set.p1().iter().find(|(_, p)| p.position == *to) {
            selections.second_selected_piece = Some(other_entity);
        }
        // second selection is a tile
        else {
            selections.second_selected_tile = Some(*to);

            // Use the first query (mutable) to try moving the piece
            if let Ok(mut moving_piece) = query_set.p0().get_mut(*piece) {
                if can_move_to_tile(&moving_piece, *to) {
                    println!("Moving piece {:?} from {:?} to {:?}", piece, from, to);
                    moving_piece.position = *to;

                    selections.selected_piece = None;
                    selections.second_selected_piece = None;
                    selections.second_selected_tile = None;
                } else {
                    println!("Invalid move for piece: {:?}", moving_piece);
                }
            }
        }
    }
}

fn can_move_to_tile(piece: &ChessPiece, to_tile: (u8, u8)) -> bool {
    // basic check for the movement rules of each piece
    // to-do: add other checks

    match piece.piece {
        PieceType::Pawn => {
            // Example: pawns move forward by 1
            let dx = to_tile.0 as i8 - piece.position.0 as i8;
            let dy = to_tile.1 as i8 - piece.position.1 as i8;
            dx == 0 && dy == 1 // Only forward by 1
        }
        PieceType::Rook => {
            // Rooks move in straight lines
            piece.position.0 == to_tile.0 || piece.position.1 == to_tile.1
        }
        PieceType::Knight => {
            // Knights move in L-shape
            let dx = (to_tile.0 as i8 - piece.position.0 as i8).abs();
            let dy = (to_tile.1 as i8 - piece.position.1 as i8).abs();
            (dx == 2 && dy == 1) || (dx == 1 && dy == 2)
        }
        PieceType::Bishop => {
            // Bishops move diagonally
            let dx = (to_tile.0 as i8 - piece.position.0 as i8).abs();
            let dy = (to_tile.1 as i8 - piece.position.1 as i8).abs();
            dx == dy
        }
        PieceType::Queen => {
            // Queens move like rook or bishop
            let dx = (to_tile.0 as i8 - piece.position.0 as i8).abs();
            let dy = (to_tile.1 as i8 - piece.position.1 as i8).abs();
            dx == dy || piece.position.0 == to_tile.0 || piece.position.1 == to_tile.1
        }
        PieceType::King => {
            // Kings move one square in any direction
            let dx = (to_tile.0 as i8 - piece.position.0 as i8).abs();
            let dy = (to_tile.1 as i8 - piece.position.1 as i8).abs();
            dx <= 1 && dy <= 1
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
