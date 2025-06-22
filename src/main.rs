use std::fmt;

fn main() {
    let mut pieces: Vec<ChessPiece> = vec![];

    let mut pawns: Vec<ChessPiece> = vec![];
    let mut knights: Vec<ChessPiece> = vec![];
    let mut bishops: Vec<ChessPiece> = vec![];
    let mut rooks: Vec<ChessPiece> = vec![];
    let mut queens: Vec<ChessPiece> = vec![];
    let mut kings: Vec<ChessPiece> = vec![];

    for i in 0..6 {
        match i {
            0 => spawn_pawns(&mut pawns),
            1 => spawn_knights(&mut knights),
            2 => spawn_rooks(&mut rooks),
            3 => spawn_bishops(&mut bishops),
            4 => spawn_queens(&mut queens),
            5 => spawn_kings(&mut kings),
            _ => println!("{} idk too much of a number.", i),
        }
    }

    pieces.append(&mut pawns);
    pieces.append(&mut knights);
    pieces.append(&mut rooks);
    pieces.append(&mut bishops);
    pieces.append(&mut queens);
    pieces.append(&mut kings);

    for piece in &pieces {
        println!("{}", piece);
    }
}

fn spawn_pawns(pawns: &mut Vec<ChessPiece>) {
    for white_pawn in 1..9 {
        let pawn = ChessPiece::new(
            &format!("Pawn {}", white_pawn),
            PieceType::Pawn,
            PieceColor::White,
            (white_pawn, 2),
            1,
        );

        pawns.push(pawn);
    }

    for black_pawn in 1..9 {
        let pawn = ChessPiece::new(
            &format!("Pawn {}", black_pawn),
            PieceType::Pawn,
            PieceColor::Black,
            (black_pawn, 7),
            1,
        );

        pawns.push(pawn);
    }
}

fn spawn_knights(knights: &mut Vec<ChessPiece>) {
    for white_knight in 1..3 {
        let pos: u8 = if white_knight == 1 { 2 } else { 7 };

        let knight = ChessPiece::new(
            &format!("Knight {}", white_knight),
            PieceType::Knight,
            PieceColor::White,
            (pos, 1),
            3,
        );

        knights.push(knight);
    }

    for black_knight in 1..3 {
        let pos: u8 = if black_knight == 1 { 2 } else { 7 };

        let knight = ChessPiece::new(
            &format!("Knight {}", black_knight),
            PieceType::Knight,
            PieceColor::Black,
            (pos, 8),
            3,
        );

        knights.push(knight);
    }
}

fn spawn_rooks(rooks: &mut Vec<ChessPiece>) {
    for white_rook in 1..3 {
        let pos: u8 = if white_rook == 1 { 1 } else { 8 };

        let rook = ChessPiece::new(
            &format!("Rook {}", white_rook),
            PieceType::Rook,
            PieceColor::White,
            (pos, 1),
            5,
        );

        rooks.push(rook);
    }

    for black_rook in 1..3 {
        let pos: u8 = if black_rook == 1 { 1 } else { 8 };

        let rook = ChessPiece::new(
            &format!("Rook {}", black_rook),
            PieceType::Rook,
            PieceColor::Black,
            (pos, 8),
            5,
        );

        rooks.push(rook);
    }
}

fn spawn_bishops(bishops: &mut Vec<ChessPiece>) {
    for white_bishop in 1..3 {
        let pos: u8 = if white_bishop == 1 { 3 } else { 6 };

        let bishop = ChessPiece::new(
            &format!("Bishop {}", white_bishop),
            PieceType::Bishop,
            PieceColor::White,
            (pos, 1),
            3,
        );

        bishops.push(bishop);
    }

    for black_bishop in 1..3 {
        let pos: u8 = if black_bishop == 1 { 3 } else { 6 };

        let bishop = ChessPiece::new(
            &format!("Bishop {}", black_bishop),
            PieceType::Bishop,
            PieceColor::Black,
            (pos, 8),
            3,
        );

        bishops.push(bishop);
    }
}

fn spawn_queens(queens: &mut Vec<ChessPiece>) {
    let queen_white = ChessPiece::new("Queen", PieceType::Queen, PieceColor::White, (4, 1), 9);
    queens.push(queen_white);

    let queen_black = ChessPiece::new("Queen", PieceType::Queen, PieceColor::Black, (4, 8), 9);
    queens.push(queen_black);
}

fn spawn_kings(kings: &mut Vec<ChessPiece>) {
    let king_white = ChessPiece::new("King", PieceType::King, PieceColor::White, (5, 1), 0);
    kings.push(king_white);

    let king_black = ChessPiece::new("King", PieceType::King, PieceColor::Black, (5, 8), 0);
    kings.push(king_black);
}

#[derive(Debug)]
struct ChessPiece {
    name: String,
    piece: PieceType,
    color: PieceColor,
    position: (u8, u8),
    value: u8,
}

#[derive(Debug)]
enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug)]
enum PieceColor {
    White,
    Black,
}

impl ChessPiece {
    fn new(
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

impl fmt::Display for ChessPiece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} {:?} at {:?}        -        {} {}",
            self.color, self.piece, self.position, self.name, self.value,
        )
    }
}
