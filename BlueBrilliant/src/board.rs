use std::mem;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
// A bitboard implementation of a chess board

const INIT_PAWNS: u64 = 1<<8 | 1<<9 | 1<<10 | 1<<11 | 1<<12 | 1<<13 | 1<<14 | 1<<15
                        | 1<<48 | 1<<49 | 1<<50 | 1<<51 | 1<<52 | 1<<53 | 1<<54 | 1<<55;

const INIT_KNIGHTS: u64 = 1<<1 | 1<<6 | 1<<57 | 1<<62;
const INIT_BISHOPS: u64 = 1<<2 | 1<<5 | 1<<58 | 1<<61 | 1<<3 | 1<<59;
const INIT_ROOKS: u64 = 1<<0 | 1<<7 | 1<<56 | 1<<63 | 1<<3 | 1<<59;
// const INIT_QUEENS: u64 = 1<<3 | 1<<59;
const INIT_KINGS: u64 = 1<<4 | 1<<60;

const INIT_WHITE: u64 = 1<<0 | 1<<1 | 1<<2 | 1<<3 | 1<<4 | 1<<5 | 1<<6 | 1<<7 | 1<<8 
                        | 1<<9 | 1<<10 | 1<<11 | 1<<12 | 1<<13 | 1<<14 | 1<<15;

const INIT_BLACK: u64 = 1<<48 | 1<<49 | 1<<50 | 1<<51 | 1<<52 | 1<<53 | 1<<54 | 1<<55 |1<<56 | 1<<57 
                        | 1<<58 | 1<<59 | 1<<60 | 1<<61 | 1<<62 | 1<<63;

// const MAX_NUM_MOVES: usize = 300;
pub const PROMOTE_LEFT: u8 = 0;
pub const PROMOTE_RIGHT: u8 = 1<<6;
pub const PROMOTE_CENTER: u8 = 1<<7;
pub const DIRECTION_MASK: u8 = 0b11000000;
pub const PIECE_MASK: u8 = 0b00110000;
pub const KNIGHT_MASK: u8 = 0;
pub const BISHOP_MASK: u8 = 1<<4;
pub const ROOK_MASK: u8 = 1<<5;
pub const QUEEN_MASK: u8 = 1<<4 | 1<<5;

const FIRST_RANK: u64 = 1<<0 | 1<<1 | 1<<2 | 1<<3 | 1<<4 | 1<<5 | 1<<6 | 1<<7; 
const SECOND_RANK: u64 = 1<<8 | 1<<9 | 1<<10 | 1<<11 | 1<<12 | 1<<13 | 1<<14 | 1<<15; 
const THIRD_RANK: u64 = 1<<16 | 1<<17 | 1<<18 | 1<<19 | 1<<20 | 1<<21 | 1<<22 | 1<<23;

const SIXTH_RANK: u64 =  1<<40 | 1<<41 | 1<<42 | 1<<43 | 1<<44 | 1<<45 | 1<<46 | 1<<47;
const SEVENTH_RANK: u64 = 1<<48 | 1<<49 | 1<<50 | 1<<51 | 1<<52 | 1<<53 | 1<<54 | 1<<55; 
const EIGTH_RANK: u64 = 1<<56 | 1<<57 | 1<<58 | 1<<59 | 1<<60 | 1<<61 | 1<<62 | 1<<63;

const A_FILE: u64 = 1<<0 | 1<<8 | 1<<16 | 1<<24 | 1<<32 | 1<<40 | 1<<48 | 1<<56;
const B_FILE: u64 = 1<<1 | 1<<9 | 1<<17 | 1<<25 | 1<<33 | 1<<41 | 1<<49 | 1<<57;
// const C_FILE: u64 = 1<<2 | 1<<10 | 1<<18 | 1<<26 | 1<<34 | 1<<42 | 1<<50 | 1<<58;
// const D_FILE: u64 = 1<<3 | 1<<11 | 1<<19 | 1<<27 | 1<<35 | 1<<43 | 1<<51 | 1<<59;
// const E_FILE: u64 = 1<<4 | 1<<12 | 1<<20 | 1<<28 | 1<<36 | 1<<44 | 1<<52 | 1<<60;
// const F_FILE: u64 = 1<<5 | 1<<13 | 1<<21 | 1<<29 | 1<<37 | 1<<45 | 1<<53 | 1<<61;
const G_FILE: u64 = 1<<6 | 1<<14 | 1<<22 | 1<<30 | 1<<38 | 1<<46 | 1<<54 | 1<<62; 
const H_FILE: u64 = 1<<7 | 1<<15 | 1<<23 | 1<<31 | 1<<39 | 1<<47 | 1<<55 | 1<<63; 

// const WHITE_LONG_DEST: u64 = 1<<2;
// const WHITE_SHORT_DEST: u64 = 1<<6;
const WHITE_KING_START_IDX: u8 = 4;
const WHITE_LONG_DEST_IDX: u8 = 2;
const WHITE_SHORT_DEST_IDX: u8 = 6;
const WHITE_LONG_EMPTY: u64 = 1<<1 | 1<<2 | 1<<3;
const WHITE_SHORT_EMPTY: u64 = 1<<5 | 1<<6;
const WHITE_LONG_KING: u64 = 1<<2 | 1<<3 | 1<<4;
const WHITE_SHORT_KING: u64 = 1<<4 | 1<<5 | 1<<6;

const NONE_ENC: u8 = 0;
const KING_ENC: u8 = 1;
const PAWN_ENC: u8 = 2;
const KNIGHT_ENC: u8 = 3;
const BISHOP_ENC: u8 = 4;
const ROOK_ENC: u8 = 5;
const QUEEN_ENC: u8 = 6;

const WHITE_ENC: u8 = 8;
const BLACK_ENC: u8 = 16;

// const BLACK_LONG_DEST: u64 = 1<<58;
// const BLACK_SHORT_DEST: u64 = 1<<62;
const BLACK_KING_START_IDX: u8 = 60;
const BLACK_LONG_DEST_IDX: u8 = 58;
const BLACK_SHORT_DEST_IDX: u8 = 62;
const BLACK_LONG_EMPTY: u64 = 1<<57 | 1<<58 | 1<<59;
const BLACK_SHORT_EMPTY: u64 = 1<<61 | 1<<62;
const BLACK_LONG_KING: u64 = 1<<58 | 1<<59 | 1<<60;
const BLACK_SHORT_KING: u64 = 1<<60 | 1<<61 | 1<<62;

const ALL_SQUARES: u64 = 0xffffffffffffffff;
// const NOT_A_FILE: u64 = !A_FILE;
// const NOT_H_FILE: u64 = !H_FILE;
const SECOND_RANK_START_IDX: u8 = 8;
const SECOND_RANK_END_IDX: u8 = 15;
// const FOURTH_RANK_START_IDX: u8 = 24;
// const FOURTH_RANK_END_IDX: u8 = 31;
// const FIFTH_RANK_START_IDX: u8 = 32;
// const FIFTH_RANK_END_IDX: u8 = 39;
const SEVENTH_RANK_START_IDX: u8 = 48;
const SEVENTH_RANK_END_IDX: u8 = 55;

// const BOARD_FILES: [u64; 8] = [A_FILE, B_FILE, C_FILE, D_FILE, E_FILE, F_FILE, G_FILE, H_FILE];
const FILES: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];    


//make constants for all num
const NORT: i8 = 8; 
const NOEA: i8 = 9; 
// const EAST: i8 = 1; 
const SOEA: i8 = -7; 
const SOUT: i8 = -8; 
const SOWE: i8 = -9; 
// const WEST: i8 = -1; 
const NOWE: i8 = 7; 


#[derive(Clone, Serialize, Deserialize)]
pub struct Board {
    pawns: u64,
    knights: u64,
    bishops: u64,
    rooks: u64,
    kings: u64,
    white: u64,
    black: u64,
    en_passant_target: u8,
    white_castle_long: bool,
    white_castle_short: bool,
    black_castle_long: bool,
    black_castle_short: bool,
    is_white_move: bool,
    position_counts: HashMap<u64, u32>,
    pub move_history: Vec<(u8, u8)>,
}

impl Board {
    // Getters
    pub fn pawns(&self) -> u64 {
        self.pawns
    }

    pub fn knights(&self) -> u64 {
        self.knights
    }

    pub fn bishops(&self) -> u64 {
        self.bishops & !self.rooks
    }

    pub fn rooks(&self) -> u64 {
        self.rooks & !self.bishops
    }
    
    pub fn queens(&self) -> u64 {
        self.rooks & self.bishops
    }
    pub fn add_queens(&mut self, queens: u64) {
        self.rooks |= queens;
        self.bishops |= queens;
    }

    pub fn kings(&self) -> u64 {
        self.kings
    }

    pub fn white(&self) -> u64 {
        self.white
    }

    pub fn black(&self) -> u64 {
        self.black
    }

    pub fn en_passant_target(&self) -> u8 {
        self.en_passant_target
    }

    pub fn white_castle_long(&self) -> bool {
        self.white_castle_long
    }

    pub fn white_castle_short(&self) -> bool {
        self.white_castle_short
    }

    pub fn black_castle_long(&self) -> bool {
        self.black_castle_long
    }

    pub fn black_castle_short(&self) -> bool {
        self.black_castle_short
    }

    pub fn is_white_move(&self) -> bool {
        self.is_white_move
    }
    pub fn position_counts(&mut self) -> &mut HashMap<u64, u32> {
        &mut self.position_counts
    }

    pub fn flip_move(&mut self) {
        self.is_white_move = !self.is_white_move;
    }
}

pub fn create_board() -> Board {
    return Board {
        pawns: INIT_PAWNS,
        knights: INIT_KNIGHTS,
        bishops: INIT_BISHOPS, 
        rooks: INIT_ROOKS,
        kings: INIT_KINGS,
        white: INIT_WHITE,
        black: INIT_BLACK,
        en_passant_target: 0,
        white_castle_long: true,
        white_castle_short: true,
        black_castle_long: true,
        black_castle_short: true,
        is_white_move: true,
        position_counts: HashMap::new(),
        move_history: Vec::new(),
    }
}
pub fn can_claim_draw(board: &Board, hash: u64) -> bool {
    if board.position_counts.contains_key(&hash) {
        return board.position_counts[&hash] >= 3;
    }
    return false;
}
fn number_of_descendents(board: &Board, depth: u8) -> u64{
    let mut current: Vec<Board> = Vec::new();
    let mut next: Vec<Board> = Vec::new();
    let mut count: u64 = 0; 
    current.push(board.clone());
    for _i in 0..depth {
        while let Some(cur_board) = current.pop() {
            next.extend(generate_legal_boards(&cur_board));
        }
        count = next.len() as u64;
        mem::swap(&mut current, &mut next);
        next.clear();
    }
    return count;
}

pub fn print_move_trees(board: &Board, depth: u8){
    let moves: Vec<u8> = generate_legal_moves(&board);
    let boards: Vec<Board> = generate_legal_boards(&board);
    let mut total: u64 = 0;
    for i in (0..moves.len()).step_by(2) {
        let cur: u64 = number_of_descendents(&boards[i/2], depth-1);
        println!("{}{} {}", map_square(moves[i]), map_square(moves[i+1]), cur);
        total += cur;
    }
    println!();
    println!("{}", total);
}

/*returns a board if move is valid else returns None (this should optimize making the move)
* checking for:
*   king is in check at end of move
*   king is in check at start of a castle
*/ 
pub fn game_over_check(board: &mut Board) -> String {
    let moves: Vec<u8> = generate_legal_moves(board);
    if moves.len() == 0{
        if is_check(board){
            if board.is_white_move{
                return "Black wins".to_string();
            } else {
                return "White wins".to_string();
            }
        } else {
            return "Stalemate".to_string();
        }
    }
    return "False".to_string();
}
// pub fn game_over_AB(board: &mut Board) -> u8 {
//     board.flip_move();
//     let moves: Vec<u8> = generate_legal_moves(board);
//     if moves.len() == 0{
//         if is_check(board){
//             board.flip_move();
//             return 1;
//         } else {
//             board.flip_move();
//             return 2;
//         }
//     }
//     return 0;
// }
fn valid_board(old_board: &Board, start: u8, end: u8) -> Option<Board> {
    let mut board: Board = simulate_move(old_board, start, end); 
    let attacks: u64 = generate_attacks(&board); 
    let moved_pieces: u64;
   
    if old_board.is_white_move {
        if start == WHITE_KING_START_IDX {
            board.white_castle_long = false;
            board.white_castle_short = false;
        }
        if start == 0 {
            board.white_castle_long = false;
        }
        if start == 7 {
            board.white_castle_short = false;
        }
        if end == 56 {
            board.black_castle_long = false;
        }
        if end == 63 {
            board.black_castle_short = false;
        }

        if old_board.white_castle_long && start == WHITE_KING_START_IDX && end == WHITE_LONG_DEST_IDX {
            if attacks & WHITE_LONG_KING != 0 { 
                return None;
            } else {
                return Some(board);
            }
        }
        if old_board.white_castle_short && start == WHITE_KING_START_IDX && end == WHITE_SHORT_DEST_IDX {
            if attacks & WHITE_SHORT_KING != 0 {
                return None;
            } else { 
                return Some(board);
            }
        }
        moved_pieces = board.white;
    } else {
        if start == BLACK_KING_START_IDX {
            board.black_castle_long = false;
            board.black_castle_short = false;
        }
        if start == 56 {
            board.black_castle_long = false;
        }
        if start == 63 {
            board.black_castle_short = false;
        }
        if end == 0 {
            board.white_castle_long = false;
        }
        if end == 7 {
            board.white_castle_short = false;
        }
        
        if old_board.black_castle_long && start == BLACK_KING_START_IDX && end == BLACK_LONG_DEST_IDX {
            if attacks & BLACK_LONG_KING != 0 { 
                return None;
            } else {
                return Some(board);
            }
        }
        if old_board.black_castle_short && start == BLACK_KING_START_IDX && end == BLACK_SHORT_DEST_IDX {
            if attacks & BLACK_SHORT_KING != 0 {
                return None;
            } else { 
                return Some(board);
            }
        } 
        moved_pieces = board.black;
    }
    if attacks & moved_pieces & board.kings != 0 {
        return None;
    }    
    return Some(board);
}

pub fn user_move(board: &mut Board, start: u8, end: u8) {
    let moves = generate_legal_moves(board);
    for chunk in moves.chunks(2){
        if chunk[0] == start && chunk[1] == end{
            make_move(board, start, end);
            return;
        }
    }
}

pub fn get_end_index(board: &Board, start: u8)-> Vec<u8>{
    let moves = generate_legal_moves(board);
    let mut ret: Vec<u8> = Vec::new();
    for chunk in moves.chunks(2){
        if chunk[0] == start{
            if is_promotion(board, start){
                let direction = chunk[1] & DIRECTION_MASK;
                if board.is_white_move(){
                    if direction == PROMOTE_LEFT{
                        ret.push(start+7);
                    } else if direction == PROMOTE_RIGHT{
                        ret.push(start+9);
                    } else {
                        ret.push(start+8);
                    }
                } else {
                    if direction == PROMOTE_LEFT{
                        ret.push(start-9);
                    } else if direction == PROMOTE_RIGHT{
                        ret.push(start-7);
                    } else {
                        ret.push(start-8);
                    }
                }
            }else{
                ret.push(chunk[1]);
            }
        }
    }
    return ret;
}

pub fn valid_move(old_board: &Board, start: u8, end: u8) -> bool {
    if start == end {
        return false;
    }else if start >= 64{
        return false;
    }
    //UNSAFE
    let board: Board = simulate_move(old_board, start, end); 
    let attacks: u64 = generate_attacks(&board); 
    let moved_pieces: u64;

    let _promotion: bool = 1<<start & SEVENTH_RANK & old_board.white() & old_board.pawns() != 0;
    if old_board.is_white_move {
        if old_board.white_castle_long && start == WHITE_KING_START_IDX && end == WHITE_LONG_DEST_IDX {
            if attacks & WHITE_LONG_KING != 0 { 
                return false;
            } else {
                return true;
            }
        }
        if old_board.white_castle_short && start == WHITE_KING_START_IDX && end == WHITE_SHORT_DEST_IDX {
            if attacks & WHITE_SHORT_KING != 0 {
                return false;

            } else { 
                return true;
            }
        }
        moved_pieces = board.white;
    } else {
        if old_board.white_castle_long && start == WHITE_KING_START_IDX && end == WHITE_LONG_DEST_IDX {
            if attacks & WHITE_LONG_KING != 0 { 
                return false;
            } else {
                return true;
            }
        }
        if old_board.black_castle_short && start == BLACK_KING_START_IDX && end == BLACK_SHORT_DEST_IDX {
            if attacks & BLACK_SHORT_KING != 0 {
                return false;
            } else { 
                return true;
            }
        } 
        moved_pieces = board.black;
    }
    if attacks & moved_pieces & board.kings != 0 {
        return false;
    }    
    return true;

}


pub fn make_move_notation(board: &mut Board, start: String, end: String){
    make_move(board, map_notation(start), map_notation(end));
}

#[inline]
fn is_capture(board: &Board, index: u8) -> bool {
    if board.is_white_move {
        return (1<<index) & board.black != 0;
    } else {
        return (1<<index) & board.white != 0;
    }
}
//if a pawn is on the last rank and the make move function calls this, then it must result in a promotion, forward is the only valid move

pub fn is_promotion(board: &Board, start: u8) -> bool {
    if board.is_white_move {
        return (1<<start) & board.white & board.pawns & SEVENTH_RANK != 0;
    } else {
        return (1<<start) & board.black & board.pawns & SECOND_RANK != 0;
    }
}
//Takes a board and does a move on that board
pub fn make_move(board: &mut Board, start: u8, end: u8) {
    board.move_history.push((start, end));
    if board.is_white_move {
        
        if 1<<start & board.white & board.pawns & SEVENTH_RANK != 0{
            //promotion
            capture_square(board, start);
            let mut index = 0;
            if end & DIRECTION_MASK == PROMOTE_LEFT {
                index = start+7;
                capture_square(board, index);
            } else if end & DIRECTION_MASK == PROMOTE_RIGHT {
                index = start+9;
                capture_square(board, index);
            } else if end & DIRECTION_MASK == PROMOTE_CENTER {
                index = start+8;
                capture_square(board, index);
            } 
            //set end
            if end & PIECE_MASK == KNIGHT_MASK {
                board.knights |= 1<<index;
            } else if end & PIECE_MASK == BISHOP_MASK {
                board.bishops |= 1<<index;
            } else if end & PIECE_MASK == ROOK_MASK {
                board.rooks |= 1<<index;
            } else {
                board.add_queens(1<<index);
            }
            board.white |= 1<<index;
            board.is_white_move = !(board.is_white_move);

            return;
        }
        //If this is an enp
        if  1<<start & board.white & board.pawns != 0 && board.en_passant_target == end && end != 0 {
            capture_square(board, end - 8);
            move_square(board, start, end);
            board.en_passant_target = 0;
        } else {
            board.en_passant_target = 0;
            
            if SECOND_RANK_START_IDX <= start && start <= SECOND_RANK_END_IDX &&
              end > start && (end - start == 16) && ((board.white & board.pawns & (1<<start)) != 0){
                board.en_passant_target = end-8;
                capture_square(board, end);
                move_square(board, start, end);
            } else if board.white_castle_long && start == WHITE_KING_START_IDX && end == WHITE_LONG_DEST_IDX {
                
                move_square(board, start, end);
                move_square(board, 0, 3);
            } else if board.white_castle_short && start == WHITE_KING_START_IDX && end == WHITE_SHORT_DEST_IDX {
                
                move_square(board, start, end);
                move_square(board, 7, 5);
            } else {
                capture_square(board, end);
                move_square(board, start, end);
            }
            if start == WHITE_KING_START_IDX {
                board.white_castle_long = false;
                board.white_castle_short = false;
            }
            if start == 0 {
                board.white_castle_long = false;
            }
            if start == 7 {
                board.white_castle_short = false;
            }
            if end == 56 {
                board.black_castle_long = false;
            }
            if end == 63 {
                board.black_castle_short = false;
            }
        }
    } else {
        if 1<<start & board.black & board.pawns & SECOND_RANK != 0 {
            //promotion
            capture_square(board, start);
            let mut index = 0;
            if end & DIRECTION_MASK == PROMOTE_RIGHT {
                index = start-9;
                capture_square(board, index);
            } else if end & DIRECTION_MASK == PROMOTE_LEFT {
                index = start-7;
                capture_square(board, index);
            } else if end & DIRECTION_MASK == PROMOTE_CENTER {
                index = start-8;
                capture_square(board, index);
            } 
            //set end
            if end & PIECE_MASK == KNIGHT_MASK {
                board.knights |= 1<<index;
            } else if end & PIECE_MASK == BISHOP_MASK {
                board.bishops |= 1<<index;
            } else if end & PIECE_MASK == ROOK_MASK {
                board.rooks |= 1<<index;
            } else {
                board.add_queens(1<<index);
            }
            board.black |= 1<<index;
            board.is_white_move = !(board.is_white_move);
            return;
        }
        if  1<<start & board.black & board.pawns != 0 && board.en_passant_target == end && end != 0 {
            capture_square(board, end + 8);
            move_square(board, start, end);
            board.en_passant_target = 0;
        } else {
            board.en_passant_target = 0;
            if SEVENTH_RANK_START_IDX <= start && start <= SEVENTH_RANK_END_IDX &&
              start > end && (start - end == 16) && (board.black & board.pawns & (1<<start)) != 0 {
                board.en_passant_target = end+8;
                capture_square(board, end);
                move_square(board, start, end);
            } else if board.black_castle_long && start == BLACK_KING_START_IDX && end == BLACK_LONG_DEST_IDX {
                
                move_square(board, start, end);
                move_square(board, 56, 59);
            } else if board.black_castle_short && start == BLACK_KING_START_IDX && end == BLACK_SHORT_DEST_IDX {
                
                move_square(board, start, end);
                move_square(board, 63, 61);
            } else {
                capture_square(board, end);
                move_square(board, start, end);
            }
            if start == BLACK_KING_START_IDX {
                board.black_castle_long = false;
                board.black_castle_short = false;
            }
            if start == 56 {
                board.black_castle_long = false;
            }
            if start == 63 {
                board.black_castle_short = false;
            }
            if end == 0 {
                board.white_castle_long = false;
            }
            if end == 7 {
                board.white_castle_short = false;
            }
            
        }
    }
    // board.position_counts.entry(board.zobrist_keys.compute_hash(board)).or_insert(0) +=1;
    board.is_white_move = !(board.is_white_move);
}


//Takes a board and returns a different board as if the move had been made on the first board
pub fn simulate_move(board: &Board, start: u8, end: u8) -> Board {
    let mut next_board: Board = board.clone();
    make_move(&mut next_board, start, end);
    return next_board;
}

pub fn print_current_moves(board: &Board) {
    let moves: Vec<u8> = generate_legal_moves(board);
    for i in 0..moves.len(){
        if i % 2 == 0 {
            print!("{} -> ", map_square(moves[i]));
        } else {
            println!("{}", map_square(moves[i]));
        }
    }
    println!("In this position there are {} moves", moves.len()/2);
}

pub fn print_pseudo_moves(board: &Board) {
    let moves: Vec<u8> = generate_all_moves(board);
    for i in 0..moves.len(){
        if i % 2 == 0 {
            print!("{} -> ", map_square(moves[i]));
        } else {
            println!("{}", map_square(moves[i]));
        }
    }
    println!("In this position there are {} moves", moves.len()/2);
}

pub fn generate_legal_moves(board: &Board) -> Vec<u8> {
    let moves: Vec<u8> = generate_all_moves(board);
    let mut legal_moves: Vec<u8> = Vec::new();
    for i in (0..moves.len()).step_by(2) { 
        if valid_move(board, moves[i], moves[i+1]){
            legal_moves.push(moves[i]);
            legal_moves.push(moves[i+1]);
        }
    }
    legal_moves
}
pub fn capture_ordering(board: &Board) -> Vec<u8> {
    let moves: Vec<u8> = generate_legal_moves(board);
    let mut legal_moves: Vec<u8> = Vec::new();
    let mut captures: Vec<u8> = Vec::new();
    for i in (0..moves.len()).step_by(2) { 
        if is_capture(board, moves[i+1]) {
            captures.push(moves[i]);
            captures.push(moves[i+1]);
        } else {
            legal_moves.push(moves[i]);
            legal_moves.push(moves[i+1]);
        }
    }
    captures.append(&mut legal_moves);
    return captures;
}
pub fn capture_moves_only(board: &Board) -> Vec<u8> {
    let moves: Vec<u8> = generate_legal_moves(board);
    let mut captures: Vec<u8> = Vec::new();
    for i in (0..moves.len()).step_by(2) { 
        if is_capture(board, moves[i+1]) {
            captures.push(moves[i]);
            captures.push(moves[i+1]);
        }
    }
    captures
}
pub fn ab_move_generation(board: &mut Board) -> Vec<u8> {
    let moves: Vec<u8> = generate_legal_moves(board);
    let mut quiet_moves: Vec<u8> = Vec::new();
    let mut checks: Vec<u8> = Vec::new();
    let mut captures_with_check: Vec<u8> = Vec::new();
    let mut captures_only: Vec<u8> = Vec::new();
    let mut ab_moves: Vec<u8> = Vec::new();
    for i in (0..moves.len()).step_by(2) { 
        let mut sim_board = simulate_move(board, moves[i], moves[i+1]);
        let capture = is_capture(board, moves[i+1]);
        let check = is_check(&mut sim_board);
        //If its a check and a capture
        if check{
            if capture {
                captures_with_check.push(moves[i]);
                captures_with_check.push(moves[i+1]);
                continue
            //just a check
            } else {
                checks.push(moves[i]);
                checks.push(moves[i+1]);
                continue
            }
        }
        //just a capture
        if capture && !check{
            captures_only.push(moves[i]);
            captures_only.push(moves[i+1]);
            continue
        //just a quiet move
        } else {
            quiet_moves.push(moves[i]);
            quiet_moves.push(moves[i+1]);
            continue
        }
    }
    ab_moves.append(&mut captures_with_check);
    ab_moves.append(&mut captures_only);
    ab_moves.append(&mut checks);
    ab_moves.append(&mut quiet_moves);
    ab_moves
}

pub fn is_check(board: &mut Board) -> bool {
    board.is_white_move = !board.is_white_move;
    let mut attacks: u64 = generate_attacks(board);
    board.is_white_move = !board.is_white_move;
    let king: u8 = king_position(board);
    attacks &= 1<<king;
    return attacks != 0;
}
pub fn calculate_mobility(board: & mut Board) -> i32 {
    let mobility_multiplier: i32 = 5;
    let mut score: i32 = 0;
    score += generate_all_moves(board).len() as i32 * mobility_multiplier;
    board.is_white_move = !board.is_white_move;
    score -= generate_all_moves(board).len() as i32 * mobility_multiplier;
    board.is_white_move = !board.is_white_move;

    score
}

pub fn generate_legal_boards(board: &Board) -> Vec<Board> {
    let moves: Vec<u8> = generate_all_moves(board);
    let mut legal_boards: Vec<Board> = Vec::new();
    
    for i in (0..moves.len()).step_by(2) { 
        if let Some(board) = valid_board(board, moves[i], moves[i+1]){
        //    print_board(&board);
            legal_boards.push(board);
        }
        //println!("-----------------------");
    }
    return legal_boards;
}

pub fn generate_all_moves(board: &Board) -> Vec<u8> {
    let mut moves: Vec<u8> = Vec::new();
    generate_pawn_moves(&board, &mut moves);
    generate_knight_moves(&board, &mut moves);
    generate_rook_moves(&board, &mut moves);    
    generate_bishop_moves(&board, &mut moves);    
    generate_king_moves(&board, &mut moves); 

    return moves; 
}

fn generate_pawn_moves(board: &Board, moves: &mut Vec<u8>) {
    let empty = !(board.black | board.white);
    if board.is_white_move{ //convert black and white into an array of len 2 and inx into that
        //Pushes and double pushes
        let mut single_push: u64 = nort(board.white & board.pawns) & empty;
        let mut double_push: u64 = nort(single_push & THIRD_RANK) & empty;
        
        while single_push != 0 {
            let idx:i8 = single_push.trailing_zeros() as i8;    
            single_push = ((ALL_SQUARES << idx) << 1) & single_push;
            if idx >= 56 {
                for i in 0..4 {
                    moves.push((idx - NORT) as u8);
                    moves.push(PROMOTE_CENTER | i << 4)
                }
            }
            else{
                moves.push((idx - NORT) as u8);
                moves.push(idx as u8);
            }
            
        }
        while double_push != 0 {
            let idx:i8 = double_push.trailing_zeros() as i8;    
            double_push = ((ALL_SQUARES << idx) << 1) & double_push;
            moves.push((idx - 2*NORT) as u8);
            moves.push(idx as u8);
        }

        //Captures
        let mut capture_left: u64 = nowe(board.white & board.pawns & !A_FILE) & board.black;
        let mut capture_right: u64 = noea(board.white & board.pawns & !H_FILE) & board.black;
        
        while capture_left != 0 {
            let idx: i8 =  capture_left.trailing_zeros() as i8;
            capture_left = ((ALL_SQUARES << idx) << 1) & capture_left;
            if idx >= 56 {
                for i in 0..4 {
                    moves.push((idx - NOWE) as u8);
                    moves.push(PROMOTE_LEFT | i << 4)
                }
            }
            else{
                moves.push((idx - NOWE) as u8);
                moves.push(idx as u8);
            }

        }
        
        while capture_right != 0 {
            let idx: i8 =  capture_right.trailing_zeros() as i8;
            capture_right = ((ALL_SQUARES << idx) << 1) & capture_right;
            if idx >= 56 {
                for i in 0..4 {
                    moves.push((idx - NOEA) as u8);
                    moves.push(PROMOTE_RIGHT | i << 4)
                }
            }
            else{
                moves.push((idx - NOEA) as u8);
                moves.push(idx as u8);
            }
        }

        //En passant
        let passant_left: u64 = nowe(board.white & board.pawns & !A_FILE) & (1<<board.en_passant_target);
        let passant_right: u64 = noea(board.white & board.pawns & !H_FILE) & (1<<board.en_passant_target);

        if passant_left != 0 {
            let idx: i8 = passant_left.trailing_zeros() as i8;
            moves.push((idx - NOWE) as u8);
            moves.push(idx as u8);
        }
        
        if passant_right != 0 {
            let idx: i8 = passant_right.trailing_zeros() as i8;
            moves.push((idx - NOEA) as u8);
            moves.push(idx as u8);
        }
    } else {
         //Pushes and double pushes
        let mut single_push: u64 = sout(board.black & board.pawns) & empty;
        let mut double_push: u64 = sout(single_push & SIXTH_RANK) & empty;

        while single_push != 0 {
            let idx:i8 = single_push.trailing_zeros() as i8;    
            single_push = ((ALL_SQUARES << idx) << 1) & single_push;
            if idx <= 7 {
                for i in 0..4 {
                    moves.push((idx - SOUT) as u8);
                    moves.push(PROMOTE_CENTER | i << 4)
                }
            }
            else{
                moves.push((idx - SOUT) as u8);
                moves.push(idx as u8);
            }
        }
        while double_push != 0 {
            let idx:i8 = double_push.trailing_zeros() as i8;    
            double_push = ((ALL_SQUARES << idx) << 1) & double_push;
            moves.push((idx - 2*SOUT) as u8);
            moves.push(idx as u8);
        }

        //Captures
        let mut capture_left: u64 = sowe(board.black & board.pawns & !A_FILE) & board.white;
        let mut capture_right: u64 = soea(board.black & board.pawns & !H_FILE) & board.white;
        
        while capture_left != 0 {
            let idx: i8 =  capture_left.trailing_zeros() as i8;
            capture_left = ((ALL_SQUARES << idx) << 1) & capture_left;
            if idx <= 7 {
                for i in 0..4 {
                    moves.push((idx - SOWE) as u8);
                    moves.push(PROMOTE_LEFT | i << 4)
                }
            }
            else{
                moves.push((idx - SOWE) as u8);
                moves.push(idx as u8);
            }
        }
        
        while capture_right != 0 {
            let idx: i8 =  capture_right.trailing_zeros() as i8;
            capture_right = ((ALL_SQUARES << idx) << 1) & capture_right;
            if idx <= 7{
                for i in 0..4 {
                    moves.push((idx - SOEA) as u8);
                    moves.push(PROMOTE_RIGHT | i << 4)
                }
            }
            else{
                moves.push((idx - SOEA) as u8);
                moves.push(idx as u8);
            }
        }

        //En passant
        let passant_left: u64 = sowe(board.black & board.pawns & !A_FILE) & (1<<board.en_passant_target);
        let passant_right: u64 = soea(board.black & board.pawns & !H_FILE) & (1<<board.en_passant_target);

        if passant_left != 0 {
            let idx: i8 = passant_left.trailing_zeros() as i8;
            moves.push((idx - SOWE) as u8);
            moves.push(idx as u8);
        }
        
        if passant_right != 0 {
            let idx: i8 = passant_right.trailing_zeros() as i8;
            moves.push((idx - SOEA) as u8);
            moves.push(idx as u8);
        }
    }
}

/*pawns are special but all other pieces move the same -- just need to change which is
    friendly pieces and which are enemy piecies
*/

//TODO Fix this to have any number of knights
//I think im going to start doing this just one at a time so theres no conditionals
//Doing these with no research so it could be shit!
fn generate_knight_moves(board: &Board, moves: &mut Vec<u8>) {
    let mut n_orig: Vec<u64> = Vec::new();
    let mut n_moves: Vec<u64> = Vec::new();
    let attack_pieces: u64;
    let _defend_pieces: u64;

    if board.is_white_move {
        let mut w_knights = board.white & board.knights;
        let mut prev_idx: u8;
        
        while w_knights != 0 {
            prev_idx = w_knights.trailing_zeros() as u8;
            n_orig.push(1 << prev_idx);
            n_moves.push(0);
            w_knights &= (ALL_SQUARES << prev_idx) << 1;
        }
        attack_pieces = board.white;
        _defend_pieces = board.black;
    } else {
        let mut b_knights = board.black & board.knights;
        let mut prev_idx: u8;
        
        while b_knights != 0 {
            prev_idx = b_knights.trailing_zeros() as u8;
            n_orig.push(1 << prev_idx);
            n_moves.push(0);
            b_knights &= (ALL_SQUARES << prev_idx) << 1;
        }
        attack_pieces = board.black;
        _defend_pieces = board.white;
    }
   
   
    
    for i in 0..n_orig.len(){
        //upleft
        n_moves[i] |= nort(nort(west(n_orig[i] & !(EIGTH_RANK | SEVENTH_RANK | A_FILE)))) & !attack_pieces;
        //upright
        n_moves[i] |= nort(nort(east(n_orig[i] & !(EIGTH_RANK | SEVENTH_RANK | H_FILE)))) & !attack_pieces;
        //rightup
        n_moves[i] |= east(east(nort(n_orig[i] & !(G_FILE | H_FILE | EIGTH_RANK)))) & !attack_pieces;
        //rightdown
        n_moves[i] |= east(east(sout(n_orig[i] & !(G_FILE | H_FILE | FIRST_RANK)))) & !attack_pieces;
        //downright
        n_moves[i] |= sout(sout(east(n_orig[i] & !(FIRST_RANK | SECOND_RANK | H_FILE)))) & !attack_pieces;
        //downleft
        n_moves[i] |= sout(sout(west(n_orig[i] & !(FIRST_RANK | SECOND_RANK | A_FILE)))) & !attack_pieces;
        //leftdown
        n_moves[i] |= west(west(sout(n_orig[i] & !(A_FILE | B_FILE | FIRST_RANK)))) & !attack_pieces;
        //leftup
        n_moves[i] |= west(west(nort(n_orig[i] & !(A_FILE | B_FILE | EIGTH_RANK)))) & !attack_pieces;

        while n_moves[i] != 0 {
            let idx:i8 =  n_moves[i].trailing_zeros() as i8;
            //n_moves[i] &= (ALL_SQUARES << idx) << 1;
            //this is the same and i should push this change to the rest oops
            n_moves[i] &=  !(1 << idx);
            moves.push(n_orig[i].trailing_zeros() as u8);
            moves.push(idx as u8);
        }
    }
}

fn generate_bishop_moves(board: &Board, moves: &mut Vec<u8>) {
    let mut bishops: u64; 
    let mut iter: u8 = 1;
    
    let attack_pieces: u64;
    let defend_pieces: u64;
    
    if board.is_white_move {
        bishops = board.white & (board.bishops);
        attack_pieces = board.white;
        defend_pieces = board.black;
    } else {
        bishops = board.black & (board.bishops);
        attack_pieces = board.black;
        defend_pieces = board.white;
    }

    while bishops != 0 {
        bishops = noea(bishops & !(EIGTH_RANK | H_FILE)) & !attack_pieces;
        
        let mut temp: u64 = bishops;
        while temp != 0 {
            let d_idx: u8 = temp.trailing_zeros() as u8;
            temp &= (ALL_SQUARES << d_idx) << 1;
            moves.push(d_idx - 9*iter);
            moves.push(d_idx as u8);
        }
        iter += 1;
        bishops = bishops & !defend_pieces;
    }
    
    bishops = attack_pieces & (board.bishops);
    iter = 1;
    while bishops != 0 {
        bishops = soea(bishops & !(FIRST_RANK | H_FILE)) & !attack_pieces;
        
        let mut temp: u64 = bishops;
        while temp != 0 {
            let d_idx: u8 = temp.trailing_zeros() as u8;
            temp &= (ALL_SQUARES << d_idx) << 1;
            moves.push(d_idx + 7*iter);
            moves.push(d_idx as u8);
        }
        iter += 1;
        bishops = bishops & !defend_pieces;
    }
    
    bishops = attack_pieces & (board.bishops);
    iter = 1;
    while bishops != 0 {
        bishops = sowe(bishops & !(FIRST_RANK | A_FILE)) & !attack_pieces;
        
        let mut temp: u64 = bishops;
        while temp != 0 {
            let d_idx: u8 = temp.trailing_zeros() as u8;
            temp &= (ALL_SQUARES << d_idx) << 1;
            moves.push(d_idx + 9*iter);
            moves.push(d_idx as u8);
        }
        iter += 1;
        bishops = bishops & !defend_pieces;
    }

    bishops = attack_pieces & (board.bishops);
    iter = 1;
    while bishops != 0 {
        bishops = nowe(bishops & !(EIGTH_RANK | A_FILE)) & !attack_pieces;
        
        let mut temp: u64 = bishops;
        while temp != 0 {
            let d_idx: u8 = temp.trailing_zeros() as u8;
            temp &= (ALL_SQUARES << d_idx) << 1;
            moves.push(d_idx - 7*iter);
            moves.push(d_idx as u8);
        }
        iter += 1;
        bishops = bishops & !defend_pieces;
    }   
}

fn generate_rook_moves(board: &Board, moves: &mut Vec<u8>) {
    let mut rooks: u64; 
    let mut iter: u8 = 1;
    
    let attack_pieces: u64;
    let defend_pieces: u64;
    
    if board.is_white_move {
        rooks = board.white & (board.rooks);
        attack_pieces = board.white;
        defend_pieces = board.black;
    } else {
        rooks = board.black & (board.rooks);
        attack_pieces = board.black;
        defend_pieces = board.white;
    }

    //nort moves
    while rooks != 0 {
        rooks = nort(rooks & !EIGTH_RANK) & !attack_pieces;
        
        let mut temp: u64 = rooks;
        while temp != 0 {
            let d_idx: u8 = temp.trailing_zeros() as u8;
            temp &= (ALL_SQUARES << d_idx) << 1;
            moves.push(d_idx - 8*iter);
            moves.push(d_idx as u8);
        }
        iter += 1;
        rooks = rooks & !defend_pieces;
    }
    
    //sout moves
    rooks = attack_pieces & (board.rooks);
    iter = 1;
    while rooks != 0 {
        rooks = sout(rooks & !FIRST_RANK) & !attack_pieces;
        
        let mut temp: u64 = rooks;
        while temp != 0 {
            let d_idx: u8 = temp.trailing_zeros() as u8;
            temp &= (ALL_SQUARES << d_idx) << 1;
            moves.push(d_idx + 8*iter);
            moves.push(d_idx as u8);
        }
        iter += 1;
        rooks = rooks & !defend_pieces;
    }
    
    //east moves
    rooks = attack_pieces & (board.rooks);
    iter = 1;
    while rooks != 0 {
        rooks = east(rooks & !H_FILE) & !attack_pieces;
        
        let mut temp: u64 = rooks;
        while temp != 0 {
            let d_idx: u8 = temp.trailing_zeros() as u8;
            temp &= (ALL_SQUARES << d_idx) << 1;
            moves.push(d_idx - iter);
            moves.push(d_idx as u8);
        }
        iter += 1;
        rooks = rooks & !defend_pieces;
    }

    //west moves
    rooks= attack_pieces & (board.rooks);
    iter = 1;
    while rooks != 0 {
        rooks = west(rooks & !A_FILE) & !attack_pieces;
        
        let mut temp: u64 = rooks;
        while temp != 0 {
            let d_idx: u8 = temp.trailing_zeros() as u8;
            temp &= (ALL_SQUARES << d_idx) << 1;
            moves.push(d_idx + iter);
            moves.push(d_idx as u8);
        }
        iter += 1;
        rooks = rooks & !defend_pieces;
    }   
}

fn generate_king_moves(board: &Board, moves: &mut Vec<u8>) {
    let king: u64;
    let attack_pieces: u64;
    let _defend_pieces: u64;
    let empty = !(board.black | board.white);
    
    if board.is_white_move {
        king = board.white & board.kings;
        attack_pieces = board.white;
        _defend_pieces = board.black;

        if board.white_castle_long && (WHITE_LONG_EMPTY & empty) == WHITE_LONG_EMPTY {
            moves.push(king.trailing_zeros() as u8);
            moves.push(WHITE_LONG_DEST_IDX);
        }
        if board.white_castle_short && (WHITE_SHORT_EMPTY & empty) == WHITE_SHORT_EMPTY {
            moves.push(king.trailing_zeros() as u8);
            moves.push(WHITE_SHORT_DEST_IDX);
        }
    } else {
        king = board.black & board.kings;
        attack_pieces = board.black;
        _defend_pieces = board.white;
        
        if board.black_castle_long && (BLACK_LONG_EMPTY & empty) == BLACK_LONG_EMPTY {
            moves.push(king.trailing_zeros() as u8);
            moves.push(BLACK_LONG_DEST_IDX);
        }
        if board.black_castle_short && (BLACK_SHORT_EMPTY & empty) == BLACK_SHORT_EMPTY {
            moves.push(king.trailing_zeros() as u8);
            moves.push(BLACK_SHORT_DEST_IDX);
        }
    }
    let k: u64 = attack_pieces & board.kings;

    let mut king_moves = 
            ( nort(k & !EIGTH_RANK) | noea(k & !(EIGTH_RANK | H_FILE)) 
            | east(k & !(H_FILE))   | soea(k & !(FIRST_RANK | H_FILE)) 
            | sout(k & !FIRST_RANK) | sowe(k & !(FIRST_RANK | A_FILE)) 
            | west(k & !A_FILE)     | nowe(k & !(EIGTH_RANK | A_FILE))) & !attack_pieces;
    
    while king_moves != 0 {
        let k_idx: u8 = king_moves.trailing_zeros() as u8;
        king_moves &= (ALL_SQUARES << k_idx) << 1;
        moves.push(king.trailing_zeros() as u8);
        moves.push(k_idx as u8);
    }
    
   
}
fn king_position(board: &Board) -> u8 {
    if board.is_white_move {
        return (board.kings & board.white).trailing_zeros() as u8;
    } else {
        return (board.kings & board.black).trailing_zeros() as u8;
    }
}
pub fn print_attacks(board: &Board) {
    print_bit_board(generate_attacks(board));
}

pub fn print_black(board: &Board) {
    print_bit_board(board.black);
}

pub fn print_white(board: &Board) {
    print_bit_board(board.white);
}

pub fn print_knights(board: &Board) {
    print_bit_board(board.knights);
}

pub fn print_en(board: &Board) {
    print_bit_board(1<<board.en_passant_target);
}



fn generate_attacks(board: &Board) -> u64 { 
    let mut attack: u64 = 0;
    let attack_pieces; 
    let _defend_pieces;
    let empty = !(board.black | board.white);
    
    if board.is_white_move {
        attack_pieces = board.white;
        _defend_pieces = board.black;
    } else {
        attack_pieces = board.black;
        _defend_pieces = board.white;
    }

    if board.is_white_move{ //convert black and white into an array of len 2 and inx into that
        let capture_left: u64 = nowe(board.white & board.pawns & !A_FILE & !EIGTH_RANK);
        let capture_right: u64 = noea(board.white & board.pawns & !H_FILE & !EIGTH_RANK);
        
        attack |= capture_left;
        attack |= capture_right;
    
    } else {
        let capture_left: u64 = sowe(board.black & board.pawns & !A_FILE & !EIGTH_RANK);
        let capture_right: u64 = soea(board.black & board.pawns & !H_FILE & !EIGTH_RANK);
        
        attack |= capture_left;
        attack |= capture_right
    }

    let mut n_orig: Vec<u64> = Vec::new();
    let mut knights = attack_pieces & board.knights;
    let mut prev_idx: u8;
    let mut _ak: u64 = 0;
    while knights != 0 {
        prev_idx = knights.trailing_zeros() as u8;
        n_orig.push(1 << prev_idx);
        knights &= ALL_SQUARES << (prev_idx) << 1;
    }
    
    for i in 0..n_orig.len(){
        attack |= nort(nort(west(n_orig[i] & !(EIGTH_RANK | SEVENTH_RANK | A_FILE))));
        attack |= nort(nort(east(n_orig[i] & !(EIGTH_RANK | SEVENTH_RANK | H_FILE))));
        attack |= east(east(nort(n_orig[i] & !(G_FILE | H_FILE | EIGTH_RANK))));
        attack |= east(east(sout(n_orig[i] & !(G_FILE | H_FILE | FIRST_RANK))));
        attack |= sout(sout(east(n_orig[i] & !(FIRST_RANK | SECOND_RANK | H_FILE))));
        attack |= sout(sout(west(n_orig[i] & !(FIRST_RANK | SECOND_RANK | A_FILE))));
        attack |= west(west(sout(n_orig[i] & !(A_FILE | B_FILE | FIRST_RANK))));
        attack |= west(west(nort(n_orig[i] & !(A_FILE | B_FILE | EIGTH_RANK))));
        
        _ak |= nort(nort(west(n_orig[i] & !(EIGTH_RANK | SEVENTH_RANK | A_FILE))));
        _ak |= nort(nort(east(n_orig[i] & !(EIGTH_RANK | SEVENTH_RANK | H_FILE))));
        _ak |= east(east(nort(n_orig[i] & !(G_FILE | H_FILE | EIGTH_RANK))));
        _ak |= east(east(sout(n_orig[i] & !(G_FILE | H_FILE | FIRST_RANK))));
        _ak |= sout(sout(east(n_orig[i] & !(FIRST_RANK | SECOND_RANK | H_FILE))));
        _ak |= sout(sout(west(n_orig[i] & !(FIRST_RANK | SECOND_RANK | A_FILE))));
        _ak |= west(west(sout(n_orig[i] & !(A_FILE | B_FILE | FIRST_RANK))));
        _ak |= west(west(nort(n_orig[i] & !(A_FILE | B_FILE | EIGTH_RANK))));
    }

    let mut bishops: u64 = attack_pieces & board.bishops; 
    
    while bishops != 0 {
        bishops = noea(bishops & !(EIGTH_RANK | H_FILE));
        attack |= bishops;
        bishops = bishops & empty;
    }
    
    bishops = attack_pieces & (board.bishops);
    while bishops != 0 {
        bishops = soea(bishops & !(FIRST_RANK | H_FILE));
        attack |= bishops; 
        bishops = bishops & empty;
    }
    
    bishops = attack_pieces & (board.bishops);
    while bishops != 0 {
        bishops = sowe(bishops & !(FIRST_RANK | A_FILE));
        attack |= bishops; 
        bishops = bishops & empty;
    }

    bishops = attack_pieces & (board.bishops);
    while bishops != 0 {
        bishops = nowe(bishops & !(EIGTH_RANK | A_FILE));
        attack |= bishops; 
        bishops = bishops & empty;
    }

    let mut rooks: u64 = attack_pieces & (board.rooks); 
    
    while rooks != 0 {
        rooks = nort(rooks & !EIGTH_RANK);
        attack |= rooks; 
        rooks = rooks & empty;
    }
    
    rooks = attack_pieces & (board.rooks);
    while rooks != 0 {
        rooks = sout(rooks & !FIRST_RANK);
        attack |= rooks; 
        rooks = rooks & empty;
    }
    
    rooks = attack_pieces & (board.rooks);
    while rooks != 0 {
        rooks = east(rooks & !H_FILE);
        attack |= rooks; 
        rooks = rooks & empty;
    }

    rooks = attack_pieces & (board.rooks);
    while rooks != 0 {
        rooks = west(rooks & !A_FILE);
        attack |= rooks; 
        rooks = rooks & empty;
    }
    let k: u64 = board.kings & attack_pieces; 
    attack |= nort(k & !EIGTH_RANK) | noea(k & !(EIGTH_RANK | H_FILE)) 
            | east(k & !(H_FILE))   | soea(k & !(FIRST_RANK | H_FILE)) 
            | sout(k & !FIRST_RANK) | sowe(k & !(FIRST_RANK | A_FILE)) 
            | west(k & !A_FILE)     | nowe(k & !(EIGTH_RANK | A_FILE));
    return attack;
}



/*
*   board loading and other utilities 
*/

pub fn load_fen(board: &mut Board, fen: &str){
    board.white = 0;
    board.black = 0;
    board.pawns = 0;
    board.knights = 0;
    board.bishops = 0;
    board.rooks = 0;
    board.is_white_move = false; 
    board.white_castle_short = false;
    board.white_castle_long = false;
    board.black_castle_short = false;
    board.black_castle_long = false;

    let fen_split: Vec<&str> = fen.split(' ').collect();
    let pieces: &str = fen_split[0];
    let turn: &str = fen_split[1];
    let castle: &str = fen_split[2];
    let passant: &str = fen_split[3];
   
    let mut square: u8 = 56;
    for i in 0..pieces.len(){
        if pieces.chars().nth(i).unwrap().is_digit(10){
            square += pieces.chars().nth(i).unwrap().to_digit(10).unwrap() as u8;
        } else if pieces.chars().nth(i).unwrap() == '/' {
            square -= 16;
        } else {
            place_piece(board, square, pieces.chars().nth(i).unwrap());
            square += 1;
        }
    }
    
    if turn.starts_with("w") {
        board.is_white_move = true; 
    }

    for c in castle.chars() {
       add_castle(board, c); 
    }

    if !passant.starts_with("-") {
        board.en_passant_target = map_notation(passant.to_string());
    }
    

}

fn place_piece(board: &mut Board, square: u8, piece: char){
    if piece.is_uppercase(){
        board.white |= 1 << square;
    } else { 
        board.black |= 1 << square;
    }
    let piece = piece.to_lowercase().next().unwrap();
    if piece == 'p' {
        board.pawns |= 1 << square;
    } else if piece == 'b' {
        board.bishops |= 1 << square;
    } else if piece == 'n' {
        board.knights |= 1 << square;
    } else if piece == 'r' {
        board.rooks |= 1 << square;
    } else if piece == 'q' {
        board.rooks |= 1 << square;
        board.bishops |= 1 << square;
    } else if piece == 'k' {
        board.kings |= 1 << square; 
    }
}

fn add_castle(board: &mut Board, c: char){
    match c {
        'K'  => board.white_castle_short = true,
        'Q'  => board.white_castle_long = true,
        'k'  => board.black_castle_short = true,
        'q'  => board.black_castle_long = true,
        '-'  => return,
        _    => todo!(),
    }
}

// Fundamental Shifts

/*
The idea of these is to shift every piece on the board in that direction.
To handle the case of overflow, before making the shift, 
all positions that could result in overflow will be zeroed out.

Eg. if nort(white_pawns) is called, the return will be a bit board 
representing positions of pawns if moved one north. 
*/


#[inline]
pub fn nort(board: u64) -> u64 {
    return board << 8;
}

#[inline]
fn noea(board: u64) -> u64 {
    return board << 9;
}

#[inline]
fn east(board: u64) -> u64 {
    return board << 1;
}

#[inline]
fn soea(board: u64) -> u64 {
    return board >> 7;
}

#[inline]
pub fn sout(board: u64) -> u64 {
    return board >> 8;
}

#[inline]
fn sowe(board: u64) -> u64 {
    return board >> 9;
}

#[inline]
fn west(board: u64) -> u64 {
    return board >> 1;
}

#[inline]
fn nowe(board: u64) -> u64 {
    return board << 7;
}
pub fn east_one(board: u64) -> u64 {
    return (board << 1) & !A_FILE;
}
pub fn noea_one(board: u64) -> u64 {
    return (board << 1) & !A_FILE;
}
pub fn soea_one(board: u64) -> u64 {
    return (board << 1) & !A_FILE;
}
pub fn west_one(board: u64) -> u64 {
    return (board << 1) & !H_FILE;
}
pub fn sowe_one(board: u64) -> u64 {
    return (board << 1) & !H_FILE;
}
pub fn nowe_one(board: u64) -> u64 {
    return (board << 1) & !H_FILE;
}
/*
*   Visualization and representation functions
*/
pub fn board_enc(board: &Board) -> Vec<u8> {
    let mut return_board: Vec<u8> = Vec::new();
    for i in 0..64 {
        return_board.push(tile_encoding(board, i));
    }
    return return_board;
}

fn tile_encoding(board: &Board, loc: u64) -> u8{
    let mask: u64 = 1 << loc;
    if mask & board.white & board.pawns != 0 {
        return WHITE_ENC+PAWN_ENC;
    } else if mask & board.white & board.rooks & board.bishops != 0 {
        return WHITE_ENC+QUEEN_ENC;
    } else if mask & board.white & board.knights != 0 {
        return WHITE_ENC+KNIGHT_ENC;
    } else if mask & board.white & board.bishops != 0 {
        return WHITE_ENC+BISHOP_ENC;
    } else if mask & board.white & board.rooks != 0 {
        return WHITE_ENC+ROOK_ENC;
    } else if mask & board.white & board.kings != 0 {
        return WHITE_ENC+KING_ENC;
    } else if mask & board.black & board.pawns != 0 {
        return BLACK_ENC+PAWN_ENC;
    } else if mask & board.black & board.rooks & board.bishops != 0 {
        return BLACK_ENC+QUEEN_ENC;
    } else if mask & board.black & board.knights != 0 {
        return BLACK_ENC+KNIGHT_ENC;
    } else if mask & board.black & board.bishops != 0 {
        return BLACK_ENC+BISHOP_ENC;
    } else if mask & board.black & board.rooks != 0 {
        return BLACK_ENC+ROOK_ENC;
    } else if mask & board.black & board.kings != 0 {
        return BLACK_ENC+KING_ENC;
    } else{
        return NONE_ENC;
    }
}
fn get_square_fen(board: &Board, loc: u64) -> char{
    let mask: u64 = 1 << loc;
    if mask & board.white & board.pawns != 0 {
        return 'P';
    } else if mask & board.white & board.rooks & board.bishops != 0 {
        return 'Q';
    } else if mask & board.white & board.knights != 0 {
        return 'N';
    } else if mask & board.white & board.bishops != 0 {
        return 'B';
    } else if mask & board.white & board.rooks != 0 {
        return 'R';
    } else if mask & board.white & board.kings != 0 {
        return 'K';
    } else if mask & board.black & board.pawns != 0 {
        return 'p';
    } else if mask & board.black & board.rooks & board.bishops != 0 {
        return 'q';
    } else if mask & board.black & board.knights != 0 {
        return 'n';
    } else if mask & board.black & board.bishops != 0 {
        return 'b';
    } else if mask & board.black & board.rooks != 0 {
        return 'r';
    } else if mask & board.black & board.kings != 0 {
        return 'k';
    } else{
        return ' ';
    }
}

//NOTE: Reading empty after call is undefined

fn capture_square(board: &mut Board, idx: u8){
    let no_square: u64 = !(1 << idx);
    board.pawns &= no_square;
    board.knights &= no_square;
    board.bishops &= no_square;
    board.rooks &= no_square;
    board.kings &= no_square;
    board.white &= no_square;
    board.black &= no_square;
}

#[inline]
fn move_bit(num: &mut u64, from: u8, to: u8){
    let mask: u64 = 1 << from;
    let bit: u64 = (*num & mask) >> from;
    *num &= !mask;
    *num |= bit<<to;
}

fn move_square(board: &mut Board, from: u8, to: u8){
    move_bit(&mut board.pawns, from, to);
    move_bit(&mut board.knights, from, to);
    move_bit(&mut board.bishops, from, to);
    move_bit(&mut board.rooks, from, to);
    move_bit(&mut board.kings, from, to);
    move_bit(&mut board.white, from, to);
    move_bit(&mut board.black, from, to);
}

pub fn print_board(board: &Board){
    for i in (0..8).rev() {
        for j in 0..8 {
            print!("{}", get_square_fen(&board, i*8+j));
        }
        println!("");
    }
    println!("White move: {}", board.is_white_move);
    
}

fn map_square(idx: u8) -> String {
    return format!("{}{}", FILES[(idx % 8) as usize], (idx / 8) + 1); 
}

fn map_notation(tile: String) -> u8 {
    return 8*((tile.chars().nth(1).unwrap() as u8) - 49) + ((tile.chars().nth(0).unwrap() as u8) - 97);
}

fn print_bit_board(num: u64) {
    for i in (0..8).rev() {
        for j in 0..8 {
            print!("{}", (num >> (i * 8 + j)) & 1);
        }
        println!();
    }
}