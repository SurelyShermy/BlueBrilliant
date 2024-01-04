use std::rc::Rc;

struct CastlingRights {
    king_side: bool,
    queen_side: bool,
}
impl CastlingRights {
    pub fn new() -> CastlingRights {
        CastlingRights {
            king_side: true,
            queen_side: true,
        }
    }
}
//Thinking that having a boardstate that contains variables that need to be copied and a gamestate that contains variables that don't need to be copied
pub struct BoardState {
    turn: bool,
    // true = white, false = black
    // made a bool since it can be stored in 1 bit
    //Deep Copy Variables
    board: [u8; 128],
    white_attack_map: [Vec<u8>; 128],
    black_attack_map: [Vec<u8>; 128],

    //0x88 board so 0-128
    //Since the board should never be
    check: bool,
    checking_squares: Rc<Vec<u8>>,

    checkmate: bool,
    stalemate: bool,
    enpassant: Option<u8>,
    pinned_pieces: Rc<Vec<u8>>,
    pinners: Rc<Vec<u8>>,

    castling_black: Rc<CastlingRights>,
    castling_white: Rc<CastlingRights>,

    white_king_pos: u8,
    black_king_pos: u8,
}
pub struct GameState {
    time_increment_on: bool,
    last_move: [u8; 4],
    move_history: Vec<String>,
    position_history: Vec<String>,
    result: Option<f32>,
}
impl BoardState {
    pub fn copy_board_state(&self) -> BoardState {
        BoardState {
            //TODO: add all variables to be copied
            turn: self.turn,
            board: self.board.clone(),
            white_attack_map: self.white_attack_map.clone(),
            black_attack_map: self.black_attack_map.clone(),

            check: self.check,

            checking_squares: Rc::clone(&self.checking_squares),

            checkmate: self.checkmate,
            stalemate: self.stalemate,
            enpassant: self.enpassant,
            pinned_pieces: Rc::clone(&self.pinned_pieces),
            pinners: Rc::clone(&self.pinners),
            castling_black: Rc::clone(&self.castling_black),
            castling_white: Rc::clone(&self.castling_white),
            white_king_pos: self.white_king_pos,
            black_king_pos: self.black_king_pos,
        }
    }
    pub fn new() -> BoardState {
        let mut board = [0; 128];

        // Place white pieces
        board[0] = WHITE | ROOK;
        board[1] = WHITE | KNIGHT;
        board[2] = WHITE | BISHOP;
        board[3] = WHITE | QUEEN;
        board[4] = WHITE | KING;
        board[5] = WHITE | BISHOP;
        board[6] = WHITE | KNIGHT;
        board[7] = WHITE | ROOK;

        // Place white pawns
        for i in 0x10..0x18 {
            board[i as usize] = WHITE | PAWN;
        }

        // Place black pawns
        for i in 0x60..0x68 {
            board[i as usize] = BLACK | PAWN;
        }

        // Place black pieces
        board[0x70] = BLACK | ROOK;
        board[0x71] = BLACK | KNIGHT;
        board[0x72] = BLACK | BISHOP;
        board[0x73] = BLACK | QUEEN;
        board[0x74] = BLACK | KING;
        board[0x75] = BLACK | BISHOP;
        board[0x76] = BLACK | KNIGHT;
        board[0x77] = BLACK | ROOK;
        let white_attack_map: Vec<Vec<u8>> = (0..128).map(|_| Vec::new()).collect();
        let black_attack_map: Vec<Vec<u8>> = (0..128).map(|_| Vec::new()).collect();
        BoardState {
            turn: true,
            board,
            white_attack_map: white_attack_map
                .try_into()
                .unwrap_or_else(|v: Vec<Vec<u8>>| {
                    panic!("Expected a Vec of length 128 but it was {}", v.len())
                }),
            black_attack_map: black_attack_map
                .try_into()
                .unwrap_or_else(|v: Vec<Vec<u8>>| {
                    panic!("Expected a Vec of length 128 but it was {}", v.len())
                }),
            check: false,
            checking_squares: Rc::new(vec![]),
            checkmate: false,
            stalemate: false,
            enpassant: None,
            pinned_pieces: Rc::new(vec![]),
            pinners: Rc::new(vec![]),
            castling_black: Rc::new(CastlingRights::new()),
            castling_white: Rc::new(CastlingRights::new()),
            white_king_pos: 4,
            black_king_pos: 0x74,
        }
    }
}
impl GameState {
    pub fn new() -> GameState {
        GameState {
            time_increment_on: false,
            last_move: [0; 4],
            move_history: vec![],
            position_history: vec![],
            result: None,
        }
    }
}

pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Direction {
    pub fn value(&self) -> isize {
        match self {
            Direction::North => 16,
            Direction::South => -16,
            Direction::East => 1,
            Direction::West => -1,
            Direction::NorthEast => 17,
            Direction::NorthWest => 15,
            Direction::SouthEast => -15,
            Direction::SouthWest => -17,
        }
    }
}

//Directions
//Uses i8s for directions since needs to be signed
const KNIGHT_MOVES: [i8; 8] = [33, 31, 18, 14, -33, -31, -18, -14];

//Pieces
//unsigned ints that allow for easy bit manipuation for determine types
const NONE: u8 = 0;
const KING: u8 = 1;
const PAWN: u8 = 2;
const KNIGHT: u8 = 3;
const BISHOP: u8 = 4;
const ROOK: u8 = 5;
const QUEEN: u8 = 6;

const WHITE: u8 = 8;
const BLACK: u8 = 16;
