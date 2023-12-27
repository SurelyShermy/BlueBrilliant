const PIECE_MAPPING: [(u8, &str); 12] = [
    (0, "Empty"),
    (9, "White king"),
    (10, "White pawn"),
    (11, "White knight"),
    (12, "White bishop"),
    (13, "White rook"),
    (14, "White queen"),
    (17, "Black king"),
    (18, "Black pawn"),
    (19, "Black knight"),
    (20, "Black bishop"),
    (21, "Black rook"),
    (22, "Black queen"),
];

const PIECE_TO_LETTER: [(u8, &str); 6] =
    [(1, "K"), (2, ""), (3, "N"), (4, "B"), (5, "R"), (6, "Q")];

fn letter_to_piece(letter: char) -> u8 {
    match letter {
        'K' => 1,
        'N' => 3,
        'B' => 4,
        'R' => 5,
        'Q' => 6,
        _ => 0, // Default case
    }
}
fn fen_to_piece(fen_char: char) -> u8 {
    match fen_char {
        'r' => BLACK | ROOK,
        'n' => BLACK | KNIGHT,
        'b' => BLACK | BISHOP,
        'q' => BLACK | QUEEN,
        'k' => BLACK | KING,
        'p' => BLACK | PAWN,
        'R' => WHITE | ROOK,
        'N' => WHITE | KNIGHT,
        'B' => WHITE | BISHOP,
        'Q' => WHITE | QUEEN,
        'K' => WHITE | KING,
        'P' => WHITE | PAWN,
        _ => 0, // Default case
    }
}
pub fn on_board(square: i16) -> bool {
    if square < 0 || square > 127 {
        return false;
    }
    let sum = square & 0x88;
    sum == 0
}

pub fn isWhite(piece: u8) -> bool {
    if piece == 0 {
        return false;
    }
    piece & WHITE == WHITE
}
pub fn isBlack(piece: u8) -> bool {
    if piece == 0 {
        return false;
    }
    piece & BLACK == BLACK
}
pub fn pieceColor(piece: u8) -> bool {
    if (isWhite(piece)) {
        return WHITE;
    }
    BLACK
}

//For coverting a 0x88 index to a decimal index
pub fn toDecIndex(square: u8) -> u8 {
    let newIndex = (square + (square & 7)) >> 1;
    newIndex
}

//For convertin a decimal index to a 0x88 index
pub fn toHexIndex(square: u8) -> u8 {
    let newIndex = (square + (square & !7));
    newIndex
}
pub fn pieceType(piece: u8) -> u8 {
    if piece == 0 {
        return 0;
    }
    piece & 0b111
}

pub fn isRookOrQueen(piece: u8) -> bool {
    if piece == 0 {
        return false;
    }
    if (pieceType(piece) == ROOK || pieceType(piece) == QUEEN) {
        return true;
    }
    false
}
pub fn isBishopOrQueen(piece: u8) -> bool {
    if piece == 0 {
        return false;
    }
    if (pieceType(piece) == BISHOP || pieceType(piece) == QUEEN) {
        return true;
    }
    false
}
pub fn isSlider(piece: u8) -> bool {
    if piece == 0 {
        return false;
    }
    if (isRookOrQueen(piece) || isBishopOrQueen(piece)) {
        return true;
    }
    false
}
pub fn calculate_path(board_state: &BoardState) {
    let mut path = Vec::new();
    let checking_square = board_state.checking_squares[0][0];
    if (piece_type(board_state.board[checking_square]) == KNIGHT) {
        path.push(checking_square);
    } else {
        let mut temp_position = checking_square;
        let vector = board_state.checking_squares[0][1];
        let player_king_pos = if (color == WHITE) {
            board_state.white_king_pos
        } else {
            board_state.black_king_pos
        };
        while (on_board(temp_position as i16) && temp_position != player_king_pos) {
            path.push(temp_position);
            temp_position += vector;
        }
    }
    return path;
}
