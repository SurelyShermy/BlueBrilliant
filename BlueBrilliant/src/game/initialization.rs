use std::rc::Rc;
struct CastlingRights {
    kingSide: bool,
    queenSide: bool,
}
//Thinking that having a boardstate that contains variables that need to be copied and a gamestate that contains variables that don't need to be copied
struct boardState {
    turn: bool,
    // true = white, false = black
    // made a bool since it can be stored in 1 bit
    //Deep Copy Variables
    board: [u8; 128],
    white_attack_map: [Vec<u8>; 64],
    black_attack_map: [Vec<u8>; 64],

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
struct gameState {
    time_increment_on: bool,
    last_move: [u8; 4],
    move_history: Vec<String>,
    position_history: Vec<String>,
    result: Option<f32>,
}
impl boardState {
    fn copyboardState(&self) -> boardState {
        boardState {
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
    fn new() -> boardState {
        let mut board = [EMPTY; 128];

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

        boardState {
            turn: true,
            board,
            white_attack_map: [vec![]; 64],
            black_attack_map: [vec![]; 64],
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
impl gameState {
    fn new() -> gameState {
        gameState {
            time_increment_on: false,
            last_move: [u8; 4],
            move_history: vec![],
            position_history: vec![],
            result: None,
        }
    }
}

//Directions
//Uses i8s for directions since needs to be signed
const NW: i8 = 15;
const NE: i8 = 17;
const N: i8 = 16;
const S: i8 = -16;
const SW: i8 = -17;
const SE: i8 = -15;
const E: i8 = 1;
const W: i8 = -1;
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
