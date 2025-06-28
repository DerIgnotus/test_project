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
    game_state: Res<GameState>,
) {
    if game_state.checkmate || game_state.stalemate {
        println!("Game over! Cannot make any moves.");
        return;
    }

    for CurrentTile(tile) in events.read() {
        println!("Tile clicked: {:?}", tile);

        // if no first piece is selected
        if selections.selected_piece.is_none() && selections.second_selected_piece.is_none() {
            if let Some((entity, _piece)) = query.iter().find(|(_, piece)| piece.position == *tile)
            {
                if game_state.turn != _piece.color {
                    println!(
                        "It's not your turn! Current turn: {:?}, piece color: {:?}",
                        game_state.turn, _piece.color
                    );
                    continue;
                }

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
    mut event_move_made: EventWriter<MoveMade>,
    mut selections: ResMut<Selections>,
    mut query_set: ParamSet<(
        Query<&mut ChessPiece>,
        Query<(Entity, &ChessPiece, &mut Transform)>,
    )>,
    mut commands: Commands,
) {
    for MovePiece { piece, from, to } in events.read() {
        // checks whether the second selection is a piece or a tile
        // second selection is a piece
        let pieces: Vec<ChessPiece> = query_set
            .p1()
            .iter()
            .map(|(_, p, _)| p.clone())
            .collect::<Vec<ChessPiece>>();

        if let Some((other_entity, other_piece_chess_piece, _)) =
            query_set.p1().iter().find(|(_, p, _)| p.position == *to)
        {
            // Extract the data needed from other_piece_chess_piece before the mutable borrow
            let other_piece_position = other_piece_chess_piece.position;
            let other_piece_color = other_piece_chess_piece.color;
            let other_piece_type = other_piece_chess_piece.piece;

            if let Ok(mut moving_piece) = query_set.p0().get_mut(*piece) {
                selections.second_selected_piece = Some(other_entity);

                let pieces_refs: Vec<&ChessPiece> = pieces.iter().collect();
                if can_move_to_tile(&moving_piece, *to, &pieces_refs, true) {
                    if is_king_in_check(&moving_piece.color, &pieces.iter().collect::<Vec<_>>()) {
                        let mut hypothetical_pieces = pieces.clone();

                        if let Some(p) = hypothetical_pieces.iter_mut().find(|p| {
                            p.position == moving_piece.position
                                && p.color == moving_piece.color
                                && p.piece == moving_piece.piece
                        }) {
                            p.position = *to;
                        }

                        if let Some(_p) = hypothetical_pieces.iter_mut().find(|p| {
                            p.position == other_piece_position
                                && p.color == other_piece_color
                                && p.piece == other_piece_type
                        }) {
                            hypothetical_pieces.retain(|p| {
                                !(p.position == other_piece_position
                                    && p.color == other_piece_color
                                    && p.piece == other_piece_type)
                            });
                        }

                        //println!("Pieces: {:?}", hypothetical_pieces);

                        let king_in_check = if is_king_in_check(
                            &moving_piece.color,
                            &hypothetical_pieces.iter().collect::<Vec<_>>(),
                        ) {
                            println!(
                                "Move puts king in check, cannot move {:?} to {:?}",
                                moving_piece, to
                            );

                            true
                        } else {
                            false
                        };

                        if king_in_check {
                            return;
                        }

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

                        commands.entity(other_entity).despawn();

                        event_move_made.write(MoveMade {});
                    } else {
                        let pos_before_moved = moving_piece.position;

                        println!("Attacking piece {:?} from {:?} to {:?}", piece, from, to);
                        moving_piece.position = *to;

                        let mut hypothetical_pieces = pieces.clone();

                        if let Some(p) = hypothetical_pieces.iter_mut().find(|p| {
                            p.position == pos_before_moved
                                && p.color == moving_piece.color
                                && p.piece == moving_piece.piece
                        }) {
                            p.position = *to;
                        }

                        // Remove the captured piece from hypothetical_pieces
                        hypothetical_pieces.retain(|p| {
                            !(p.position == other_piece_position
                                && p.color == other_piece_color
                                && p.piece == other_piece_type)
                        });

                        let is_king_in_check = is_king_in_check(
                            &moving_piece.color,
                            &hypothetical_pieces.iter().collect::<Vec<_>>(),
                        );

                        println!("Is king in check after move? {}", is_king_in_check);

                        if is_king_in_check {
                            // If the king is in check after the move, revert the move
                            println!("Move puts king in check, reverting.");
                            moving_piece.position = pos_before_moved;

                            // Optionally, you could notify the player or handle this case differently
                            selections.selected_piece = None;
                            selections.second_selected_piece = None;
                            selections.second_selected_tile = None;
                            continue;
                        }

                        // Also update the transform position
                        if let Ok((_, _, mut transform)) = query_set.p1().get_mut(*piece) {
                            let (x, y) = tile_to_screen_coord(*to);
                            transform.translation.x = x;
                            transform.translation.y = y;
                        }

                        selections.selected_piece = None;
                        selections.second_selected_piece = None;
                        selections.second_selected_tile = None;

                        commands.entity(other_entity).despawn();

                        event_move_made.write(MoveMade {});
                    }
                }
            }
        }
        // second selection is a tile
        else {
            selections.second_selected_tile = Some(*to);

            // Use the first query (mutable) to try moving the piece
            if let Ok(mut moving_piece) = query_set.p0().get_mut(*piece) {
                if can_move_to_tile(
                    &moving_piece,
                    *to,
                    &pieces.iter().collect::<Vec<_>>(),
                    false,
                ) {
                    let pos_before_moved = moving_piece.position;

                    println!("Moving piece {:?} from {:?} to {:?}", piece, from, to);
                    moving_piece.position = *to;

                    let mut hypothetical_pieces = pieces.clone();

                    if let Some(p) = hypothetical_pieces.iter_mut().find(|p| {
                        p.position == pos_before_moved
                            && p.color == moving_piece.color
                            && p.piece == moving_piece.piece
                    }) {
                        p.position = *to;
                    }

                    let is_king_in_check = is_king_in_check(
                        &moving_piece.color,
                        &hypothetical_pieces.iter().collect::<Vec<_>>(),
                    );

                    println!("Is king in check after move? {}", is_king_in_check);

                    if is_king_in_check {
                        // If the king is in check after the move, revert the move
                        println!("Move puts king in check, reverting.");
                        moving_piece.position = pos_before_moved;

                        // Optionally, you could notify the player or handle this case differently
                        selections.selected_piece = None;
                        selections.second_selected_piece = None;
                        selections.second_selected_tile = None;
                        continue;
                    }

                    // Also update the transform position
                    if let Ok((_, _, mut transform)) = query_set.p1().get_mut(*piece) {
                        let (x, y) = tile_to_screen_coord(*to);
                        transform.translation.x = x;
                        transform.translation.y = y;
                    }

                    selections.selected_piece = None;
                    selections.second_selected_piece = None;
                    selections.second_selected_tile = None;

                    event_move_made.write(MoveMade {});
                } else {
                    selections.second_selected_piece = None;
                    selections.second_selected_tile = None;

                    println!("Invalid move for piece: {:?}", moving_piece);
                }
            }
        }
    }
}

pub fn move_made(
    mut event_move_made: EventReader<MoveMade>,
    mut game_state: ResMut<GameState>,
    query: Query<(Entity, &ChessPiece)>,
) {
    for MoveMade {} in event_move_made.read() {
        println!("Move made event triggered.");

        game_state.turn = match game_state.turn {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        };

        let pieces: Vec<&ChessPiece> = query.iter().map(|(_, p)| p).collect();

        if is_king_in_check(&game_state.turn, &pieces) {
            game_state.check = true;

            if has_free_squares(query, &game_state.turn)
                || can_capture_attacker(query, &game_state.turn)
            {
                game_state.checkmate = false;
            } else if !can_capture_attacker(query, &game_state.turn) {
                game_state.checkmate = true;
            }
        } else {
            game_state.check = false;

            let only_king_exists = pieces
                .iter()
                .filter(|p| p.color == game_state.turn && p.piece != PieceType::King)
                .count()
                == 0;

            if has_free_squares(query, &game_state.turn) {
                game_state.stalemate = false;
            } else if only_king_exists {
                game_state.stalemate = true;
            }
        }
    }
}

fn can_capture_attacker(pieces_query: Query<(Entity, &ChessPiece)>, color: &PieceColor) -> bool {
    let pieces: Vec<ChessPiece> = pieces_query.iter().map(|(_, p)| p.clone()).collect();

    // Get all enemy pieces
    let enemy_pieces: Vec<ChessPiece> = pieces
        .iter()
        .filter(|p| p.color != *color)
        .cloned()
        .collect();

    // Get all friendly pieces
    let friendly_pieces: Vec<ChessPiece> = pieces
        .iter()
        .filter(|p| p.color == *color)
        .cloned()
        .collect();

    // Find the king
    let king = match pieces
        .iter()
        .find(|p| p.color == *color && p.piece == PieceType::King)
    {
        Some(k) => k,
        None => return false,
    };

    // Identify the piece(s) attacking the king
    let attackers: Vec<&ChessPiece> = enemy_pieces
        .iter()
        .filter(|enemy| can_move_to_tile(enemy, king.position, &pieces.iter().collect(), true))
        .collect();

    if attackers.is_empty() {
        return false; // Not in check
    }

    // If multiple attackers, can't capture all in one move
    if attackers.len() > 1 {
        return false;
    }

    let attacker = attackers[0];

    // Try capturing the attacker with any friendly piece (except the king)
    for piece in friendly_pieces
        .iter()
        .filter(|p| p.piece != PieceType::King)
    {
        if can_move_to_tile(piece, attacker.position, &pieces.iter().collect(), false) {
            // Simulate capture
            let mut hypothetical_pieces = pieces.clone();

            if let Some(p) = hypothetical_pieces
                .iter_mut()
                .find(|p| p.position == piece.position)
            {
                p.position = attacker.position; // move to attacker's tile
            }

            // Remove the attacker
            hypothetical_pieces.retain(|p| p.position != attacker.position || p.color == *color);

            if !is_king_in_check(color, &hypothetical_pieces.iter().collect::<Vec<_>>()) {
                println!("Capture attacker at {:?}", attacker.position);
                return true;
            }
        }
    }

    false
}

fn has_free_squares(pieces_query: Query<(Entity, &ChessPiece)>, color: &PieceColor) -> bool {
    // Find all pieces of the given color
    let pieces: Vec<ChessPiece> = pieces_query.iter().map(|(_, p)| p.clone()).collect();

    // Find the king of the given color
    let king = match pieces
        .iter()
        .find(|p| p.piece == PieceType::King && p.color == *color)
    {
        Some(k) => k,
        None => return false, // No king found, can't have free squares
    };

    for x in 1..=8 {
        for y in 1..=8 {
            let to_tile = (x, y);

            if let Some(target_piece) = pieces.iter().find(|p| p.position == to_tile) {
                if target_piece.color == *color {
                    continue;
                }
            }

            if can_move_to_tile(king, to_tile, &pieces.iter().collect(), false) {
                let mut hypothetical_pieces = pieces.clone();

                if let Some(p) = hypothetical_pieces.iter_mut().find(|p| {
                    p.position == king.position && p.color == king.color && p.piece == king.piece
                }) {
                    p.position = to_tile;
                }

                if let Some(_target_piece) = pieces.iter().find(|p| p.position == to_tile) {
                    hypothetical_pieces.retain(|p| p.position != to_tile || p.color == *color);
                }

                //println!("Hypothetical pieces after move: {:?}", hypothetical_pieces);

                let is_king_in_check =
                    is_king_in_check(color, &hypothetical_pieces.iter().collect::<Vec<_>>());

                if is_king_in_check {
                    continue;
                }

                println!("Found free square at {:?}", to_tile);

                return true;
            }
        }
    }

    false
}

fn can_move_to_tile(
    piece: &ChessPiece,
    to_tile: (u8, u8),
    pieces: &Vec<&ChessPiece>,
    attacking: bool,
) -> bool {
    let is_occupied = |pos: (u8, u8)| pieces.iter().any(|p| p.position == pos);

    match piece.piece {
        PieceType::Pawn => can_pawn_move(piece, to_tile, pieces, attacking),
        PieceType::Rook => can_rook_move(piece, to_tile, pieces, &is_occupied),
        PieceType::Bishop => can_bishop_move(piece, to_tile, pieces, &is_occupied),
        PieceType::Queen => can_queen_move(piece, to_tile, pieces, &is_occupied),
        PieceType::Knight => can_knight_move(piece, to_tile, pieces),
        PieceType::King => can_king_move(piece, to_tile, pieces),
    }
}

fn is_king_in_check(color: &PieceColor, pieces: &Vec<&ChessPiece>) -> bool {
    // 1. Find the king of the given color
    let king = match pieces
        .iter()
        .find(|p| p.piece == PieceType::King && p.color == *color)
    {
        Some(k) => k,
        None => return false, // No king found, can't be in check
    };
    let king_pos = king.position;

    // 2. For each enemy piece, check if it can move to the king's position
    for piece in pieces.iter().filter(|p| p.color != *color) {
        let can_attack = match piece.piece {
            PieceType::Pawn => can_pawn_move(piece, king_pos, pieces, false),
            PieceType::Rook => can_rook_move(piece, king_pos, pieces, &|pos| {
                pieces.iter().any(|p| p.position == pos)
            }),
            PieceType::Bishop => can_bishop_move(piece, king_pos, pieces, &|pos| {
                pieces.iter().any(|p| p.position == pos)
            }),
            PieceType::Queen => can_queen_move(piece, king_pos, pieces, &|pos| {
                pieces.iter().any(|p| p.position == pos)
            }),
            PieceType::Knight => can_knight_move(piece, king_pos, pieces),
            PieceType::King => {
                let dx = (king_pos.0 as i8 - piece.position.0 as i8).abs();
                let dy = (king_pos.1 as i8 - piece.position.1 as i8).abs();
                dx <= 1 && dy <= 1
            }
        };
        if can_attack {
            return true;
        }
    }
    false
}

pub fn update_ui(mut text_query: Query<&mut Text>, game_state: Res<GameState>) {
    for mut text in text_query.iter_mut() {
        *text = format!(
            "Turn: {:?}\nCheck: {:?}\nCheckmate: {:?}\nStalemate: {:?}",
            game_state.turn, game_state.check, game_state.checkmate, game_state.stalemate
        )
        .into();
    }
}

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

// Can Moves

fn can_king_move(piece: &ChessPiece, to_tile: (u8, u8), pieces: &Vec<&ChessPiece>) -> bool {
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

fn can_knight_move(piece: &ChessPiece, to_tile: (u8, u8), pieces: &Vec<&ChessPiece>) -> bool {
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

fn can_queen_move(
    piece: &ChessPiece,
    to_tile: (u8, u8),
    pieces: &Vec<&ChessPiece>,
    is_occupied: &dyn Fn((u8, u8)) -> bool,
) -> bool {
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

fn can_bishop_move(
    piece: &ChessPiece,
    to_tile: (u8, u8),
    pieces: &Vec<&ChessPiece>,
    is_occupied: &dyn Fn((u8, u8)) -> bool,
) -> bool {
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

fn can_rook_move(
    piece: &ChessPiece,
    to_tile: (u8, u8),
    pieces: &Vec<&ChessPiece>,
    is_occupied: &dyn Fn((u8, u8)) -> bool,
) -> bool {
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

fn can_pawn_move(
    piece: &ChessPiece,
    to_tile: (u8, u8),
    pieces: &Vec<&ChessPiece>,
    attacking: bool,
) -> bool {
    if attacking {
        return can_pawn_attack(piece, to_tile, pieces);
    }

    let dy = to_tile.1 as i8 - piece.position.1 as i8;
    let dx = (to_tile.0 as i8 - piece.position.0 as i8).abs();

    match piece.color {
        PieceColor::White => {
            // White pawns move up (increasing y)
            if piece.position.1 == 2 && dy == 2 && dx == 0 {
                // First move, two squares forward
                let intermediate = (piece.position.0, piece.position.1 + 1);
                if !pieces.iter().any(|p| p.position == intermediate)
                    && !pieces.iter().any(|p| p.position == to_tile)
                {
                    return true;
                }
            } else if dy == 1 && dx == 0 {
                // Normal move, one square forward
                if !pieces.iter().any(|p| p.position == to_tile) {
                    return true;
                }
            } else if dy == 1 && dx == 1 {
                // Capture diagonally
                if let Some(target) = pieces.iter().find(|p| p.position == to_tile) {
                    return target.color != piece.color;
                }
            }
        }
        PieceColor::Black => {
            // Black pawns move down (decreasing y)
            if piece.position.1 == 7 && dy == -2 && dx == 0 {
                // First move, two squares forward
                let intermediate = (piece.position.0, piece.position.1 - 1);
                if !pieces.iter().any(|p| p.position == intermediate)
                    && !pieces.iter().any(|p| p.position == to_tile)
                {
                    return true;
                }
            } else if dy == -1 && dx == 0 {
                // Normal move, one square forward
                if !pieces.iter().any(|p| p.position == to_tile) {
                    return true;
                }
            } else if dy == -1 && dx == 1 {
                // Capture diagonally
                if let Some(target) = pieces.iter().find(|p| p.position == to_tile) {
                    return target.color != piece.color;
                }
            }
        }
    }
    false
}

fn can_pawn_attack(piece: &ChessPiece, to_tile: (u8, u8), pieces: &Vec<&ChessPiece>) -> bool {
    let dy = to_tile.1 as i8 - piece.position.1 as i8;
    let dx = (to_tile.0 as i8 - piece.position.0 as i8).abs();

    (match piece.color {
        PieceColor::White => dy == 1 && dx == 1,
        PieceColor::Black => dy == -1 && dx == 1,
    }) && pieces
        .iter()
        .any(|p| p.position == to_tile && p.color != piece.color)
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
