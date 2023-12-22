struct attackMaps {
    whiteAttackMap: [Vec<u8>; 128],
    blackAttackMap: [Vec<u8>; 128],
}
struct GameState {
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
    checking_squares: Vec<u8>,

    checkmate: bool,
    stalemate: bool,
    enpassant: Option<u8>,
    pinned_pieces: Vec<u8>,
    pinners: Vec<u8>,
    time_increment_on: bool,
    castling_black: CastlingRights,
    castling_white: CastlingRights,

    white_king_pos: u8,
    black_king_pos: u8,

    last_move: [u8; 4],
    move_history: Vec<String>,
    position_history: Vec<String>,
    captured_piece: u8,
    guest: bool,
    result: Option<f32>,
}

impl GameState {
    fn set_enpassant_square(&mut self, square: Option<u8>) {
        if Rc::strong_count(&self.enpassant_square) > 1 {
            // If there are other Rc pointers to this data, clone the data to create a new instance
            self.enpassant_square = Rc::new(square);
        } else {
            // Safe to modify directly
            Rc::make_mut(&mut self.enpassant_square).replace(square);
        }
    }
    fn new() -> GameState {
        GameState {
            turn: true,

            board: [0; 128],
            check: false,
            checking_squares: vec![64],
            checkmate: false,
            stalemate: false,
            enpassant: None,
            pinned_pieces: vec![],
            pinners: vec![],
            time_increment_on: false,
            castling_black: CastlingRights::new(),
            castling_white: CastlingRights::new(),
            white_king_pos: 0,
            black_king_pos: 0,
            last_move: [0; 4],
            move_history: vec![],
            position_history: vec![],
            captured_piece: 0,
            result: None,
        }
    }
}
//Directions
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
const NONE: u8 = 0;
const KING: u8 = 1;
const PAWN: u8 = 2;
const KNIGHT: u8 = 3;
const BISHOP: u8 = 4;
const ROOK: u8 = 5;
const QUEEN: u8 = 6;

const WHITE: u8 = 8;
const BLACK: u8 = 16;
