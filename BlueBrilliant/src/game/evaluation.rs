
use board::*;
// use board::nort;
// use board::sout;
// use board::east_one;
// use board::west_one;

//Piece Square tables

const PAWN_VALUE: i32 = 100;
const KNIGHT_VALUE: i32 = 320;
const BISHOP_VALUE: i32 = 330;
const ROOK_VALUE: i32 = 500;
const QUEEN_VALUE: i32 = 900;
const KING_VALUE: i32 = 20000;

const MG_PASSED_PAWN: i32 = 25;
const EG_PASSED_PAWN: i32 = 50;
const BROKEN_PAWN_SHELTER: i32 = -50;
const ROOK_OPEN_FILE: i32 = 30;
const ROOK_SEMI_FILE: i32 = 5;
const MG_PAWN_TABLE: [i32; 64] = [
     0,   0,   0,   0,   0,   0,  0,   0,
    98, 134,  61,  95,  68, 126, 34, -11,
    -6,   7,  26,  31,  65,  56, 25, -20,
   -14,  13,   6,  21,  23,  12, 17, -23,
   -27,  -2,  -5,  12,  17,   6, 10, -25,
   -26,  -4,  -4, -10,   3,   3, 33, -12,
   -35,  -1, -20, -23, -15,  24, 38, -22,
     0,   0,   0,   0,   0,   0,  0,   0,
];

const EG_PAWN_TABLE: [i32; 64] = [
     0,   0,   0,   0,   0,   0,   0,   0,
   178, 173, 158, 134, 147, 132, 165, 187,
    94, 100,  85,  67,  56,  53,  82,  84,
    32,  24,  13,   5,  -2,   4,  17,  17,
    13,   9,  -3,  -7,  -7,  -8,   3,  -1,
     4,   7,  -6,   1,   0,  -5,  -1,  -8,
    13,   8,   8,  10,  13,   0,   2,  -7,
     0,   0,   0,   0,   0,   0,   0,   0,
];
const MG_KNIGHT_TABLE: [i32; 64] = [
  -167, -89, -34, -49,  61, -97, -15, -107,
   -73, -41,  72,  36,  23,  62,   7,  -17,
   -47,  60,  37,  65,  84, 129,  73,   44,
    -9,  17,  19,  53,  37,  69,  18,   22,
   -13,   4,  16,  13,  28,  19,  21,   -8,
   -23,  -9,  12,  10,  19,  17,  25,  -16,
   -29, -53, -12,  -3,  -1,  18, -14,  -19,
  -105, -21, -58, -33, -17, -28, -19,  -23,
];


const EG_KNIGHT_TABLE: [i32; 64] = [
    -58, -38, -13, -28, -31, -27, -63, -99,
    -25,  -8, -25,  -2,  -9, -25, -24, -52,
    -24, -20,  10,   9,  -1,  -9, -19, -41,
    -17,   3,  22,  22,  22,  11,   8, -18,
    -18,  -6,  16,  25,  16,  17,   4, -18,
    -23,  -3,  -1,  15,  10,  -3, -20, -22,
    -42, -20, -10,  -5,  -2, -20, -23, -44,
    -29, -51, -23, -15, -22, -18, -50, -64,
];

const MG_BISHOP_TABLE: [i32; 64] = [
  -29,   4, -82, -37, -25, -42,   7,  -8,
  -26,  16, -18, -13,  30,  59,  18, -47,
  -16,  37,  43,  40,  35,  50,  37,  -2,
   -4,   5,  19,  50,  37,  37,   7,  -2,
   -6,  13,  13,  26,  34,  12,  10,   4,
    0,  15,  15,  15,  14,  27,  18,  10,
    4,  15,  16,   0,   7,  21,  33,   1,
  -33,  -3, -14, -21, -13, -12, -39, -21,
];

const EG_BISHOP_TABLE: [i32; 64] = [
  -14, -21, -11,  -8, -7,  -9, -17, -24,
   -8,  -4,   7, -12, -3, -13,  -4, -14,
    2,  -8,   0,  -1, -2,   6,   0,   4,
   -3,   9,  12,   9, 14,  10,   3,   2,
   -6,   3,  13,  19,  7,  10,  -3,  -9,
  -12,  -3,   8,  10, 13,   3,  -7, -15,
  -14, -18,  -7,  -1,  4,  -9, -15, -27,
  -23,  -9, -23,  -5, -9, -16,  -5, -17,
];

const MG_ROOK_TABLE: [i32; 64] = [
   32,  42,  32,  51, 63,  9,  31,  43,
   27,  32,  58,  62, 80, 67,  26,  44,
   -5,  19,  26,  36, 17, 45,  61,  16,
  -24, -11,   7,  26, 24, 35,  -8, -20,
  -36, -26, -12,  -1,  9, -7,   6, -23,
  -45, -25, -16, -17,  3,  0,  -5, -33,
  -44, -16, -20,  -9, -1, 11,  -6, -71,
  -19, -13,   1,  17, 16,  7, -37, -26,
];

const EG_ROOK_TABLE: [i32; 64] = [
  13, 10, 18, 15, 12,  12,   8,   5,
  11, 13, 13, 11, -3,   3,   8,   3,
   7,  7,  7,  5,  4,  -3,  -5,  -3,
   4,  3, 13,  1,  2,   1,  -1,   2,
   3,  5,  8,  4, -5,  -6,  -8, -11,
  -4,  0, -5, -1, -7, -12,  -8, -16,
  -6, -6,  0,  2, -9,  -9, -11,  -3,
  -9,  2,  3, -1, -5, -13,   4, -20,
];

const MG_QUEEN_TABLE: [i32; 64] = [
  -28,   0,  29,  12,  59,  44,  43,  45,
  -24, -39,  -5,   1, -16,  57,  28,  54,
  -13, -17,   7,   8,  29,  56,  47,  57,
  -27, -27, -16, -16,  -1,  17,  -2,   1,
   -9, -26,  -9, -10,  -2,  -4,   3,  -3,
  -14,   2, -11,  -2,  -5,   2,  14,   5,
  -35,  -8,  11,   2,   8,  15,  -3,   1,
   -1, -18,  -9,  10, -15, -25, -31, -50,
];

const EG_QUEEN_TABLE: [i32; 64] = [
   -9,  22,  22,  27,  27,  19,  10,  20,
  -17,  20,  32,  41,  58,  25,  30,   0,
  -20,   6,   9,  49,  47,  35,  19,   9,
    3,  22,  24,  45,  57,  40,  57,  36,
  -18,  28,  19,  47,  31,  34,  39,  23,
  -16, -27,  15,   6,   9,  17,  10,   5,
  -22, -23, -30, -16, -16, -23, -36, -32,
  -33, -28, -22, -43,  -5, -32, -20, -41,
];

const MG_KING_TABLE: [i32; 64] = [
  -65,  23,  16, -15, -56, -34,   2,  13,
   29,  -1, -20,  -7,  -8,  -4, -38, -29,
   -9,  24,   2, -16, -20,   6,  22, -22,
  -17, -20, -12, -27, -30, -25, -14, -36,
  -49,  -1, -27, -39, -46, -44, -33, -51,
  -14, -14, -22, -46, -44, -30, -15, -27,
    1,   7,  -8, -64, -43, -16,   9,   8,
  -15,  36,  12, -54,   8, -28,  24,  14,
];

const EG_KING_TABLE: [i32; 64] = [
  -74, -35, -18, -18, -11,  15,   4, -17,
  -12,  17,  14,  17,  17,  38,  23,  11,
   10,  17,  23,  15,  20,  45,  44,  13,
   -8,  22,  24,  27,  26,  33,  26,   3,
  -18,  -4,  21,  24,  27,  23,   9, -11,
  -19,  -3,  11,  21,  23,  16,   7,  -9,
  -27, -11,   4,  13,  14,   4,  -5, -17,
  -53, -34, -21, -11, -28, -14, -24, -43
];

//File utility functions from https://www.chessprogramming.org/Pawn_Fills
pub fn north_fill(pawns: u64) -> u64 {
  let mut pawns = pawns;
  pawns |= pawns << 8;
  pawns |= pawns << 16;
  pawns |= pawns << 32;
  pawns
}

pub fn south_fill(pawns: u64)-> u64 {
  let mut pawns = pawns;
  pawns |= pawns >> 8;
  pawns |= pawns >> 16;
  pawns |= pawns >> 32;
  pawns
}
fn flip_index(index: usize) -> usize {
  index ^ 56
}
pub fn white_front_span(wpawns: u64)-> u64{
  nort(north_fill(wpawns))
}

pub fn black_rear_span(bpawns: u64)-> u64{
  nort(north_fill(bpawns))
}

pub fn black_front_span(bpawns: u64)-> u64{
  sout(south_fill(bpawns))
}

pub fn white_rear_span(wpawns: u64)-> u64{
  sout(south_fill(wpawns))
}
pub fn file_fill(pawns: u64) -> u64 {
  north_fill(pawns) | south_fill(pawns)
}
pub fn w_pawns_behind(wpawns: u64) -> u64 {
  wpawns & white_rear_span(wpawns)
}
pub fn w_pawns_front(wpawns: u64) -> u64 {
  wpawns & white_front_span(wpawns)
}
//These are setwise, they simply count the number of passed pawns
pub fn w_passed_pawn_count(wpawns: u64, bpawns: u64)->i32{
  let mut all_front_spans = black_front_span(bpawns);
  all_front_spans |= east_one(all_front_spans) | west_one(all_front_spans);
  (wpawns & !all_front_spans).count_ones() as i32
}
pub fn b_passed_pawn_count(wpawns: u64, bpawns: u64)->i32{
  let mut all_front_spans = white_front_span(wpawns);
  all_front_spans |= east_one(all_front_spans) | west_one(all_front_spans);
  (bpawns & !all_front_spans).count_ones() as i32
}

//Not sure if we are going to need this
pub fn triple_pawns_count(wpawns: u64) -> u32 {
  let pawns_ahead_own: u64 = wpawns & white_front_span(wpawns);
  let pawns_behind_own: u64 = wpawns & white_rear_span(wpawns);
  let pawns_ahead_and_behind: u64 = pawns_ahead_own & pawns_behind_own;
  pawns_ahead_and_behind.count_ones()
}

pub fn evaluate_board(board: &Board) -> i32 {
  let mut score: i32 = 0;
  //Perhaps calculate material can also pass the eg phase to allow for phase tapering
  score += calculate_material(board);
  score += calculate_pawn_structure(board, 0);
  score += calculate_king_safety(board);
  //IF both sides need to have their mobility calculated, then it would perhaps require the board to be mutable to flip the side to move? is it worth it?
  //score += calculate_mobility(board);
  score += calculate_bishop_pair(board);

  // score += rook_on_open_file(board);
  // score += rook_on_semi_open_file(board);

  score
}

pub fn calculate_material(board: &Board) -> i32 {
  //These numbers are based on pesto game phase increment count
  let knight = 1;
  let bishop = 1;
  let rook = 2;
  let queen = 4;

  let mut mgscore: i32 = 0;
  let mut egscore: i32 = 0;
  let mut game_phase: i32 = 0;
  let mut wpawns = board.pawns & board.white;
  let mut bpawns = board.pawns & board.black;
  while wpawns != 0 {
    let index = wpawns.trailing_zeros() as usize;
    mgscore += PAWN_VALUE + MG_PAWN_TABLE[index];
    egscore += PAWN_VALUE + EG_PAWN_TABLE[index];
    wpawns &= wpawns - 1;
  }
  while bpawns != 0 {
    let index = bpawns.trailing_zeros() as usize;
    mgscore -= PAWN_VALUE + MG_PAWN_TABLE[flip_index(index)];
    egscore -= PAWN_VALUE + EG_PAWN_TABLE[flip_index(index)];
    bpawns &= bpawns - 1;
  }
  let mut wknights = board.knights & board.white;
  let mut bknights = board.knights & board.black;
  while wknights != 0 {
    let index = wknights.trailing_zeros() as usize;
    mgscore += KNIGHT_VALUE + MG_KNIGHT_TABLE[index];
    egscore += KNIGHT_VALUE + EG_KNIGHT_TABLE[index];
    game_phase += knight;
    wknights &= wknights - 1;
  }
  while bknights != 0 {
    let index = bknights.trailing_zeros() as usize;
    mgscore -= KNIGHT_VALUE + MG_KNIGHT_TABLE[flip_index(index)];
    egscore -= KNIGHT_VALUE + EG_KNIGHT_TABLE[flip_index(index)];
    game_phase += knight;
    bknights &= bknights - 1;
  }
  let mut wbishops = board.bishops & board.white;
  let mut bbishops = board.bishops & board.black;
  while wbishops != 0 {
    let index = wbishops.trailing_zeros() as usize;
    mgscore += BISHOP_VALUE + MG_BISHOP_TABLE[index];
    egscore += BISHOP_VALUE + EG_BISHOP_TABLE[index];
    game_phase += bishop;
    wbishops &= wbishops - 1;
  }
  while bbishops != 0 {
    let index = bbishops.trailing_zeros() as usize;
    mgscore -= BISHOP_VALUE + MG_BISHOP_TABLE[flip_index(index)];
    egscore -= BISHOP_VALUE + EG_BISHOP_TABLE[flip_index(index)];
    game_phase += bishop;
    bbishops &= bbishops - 1;
  }
  let mut wrooks = board.rooks & board.white;
  let mut brooks = board.rooks & board.black;
  while wrooks != 0 {
    let index = wrooks.trailing_zeros() as usize;
    mgscore += ROOK_VALUE + MG_ROOK_TABLE[index];
    egscore += ROOK_VALUE + EG_ROOK_TABLE[index];
    game_phase += rook;
    wrooks &= wrooks - 1;
  }
  while brooks != 0 {
    let index = brooks.trailing_zeros() as usize;
    mgscore -= ROOK_VALUE + MG_ROOK_TABLE[flip_index(index)];
    egscore -= ROOK_VALUE + EG_ROOK_TABLE[flip_index(index)];
    game_phase += rook;
    brooks &= brooks - 1;
  }
  let mut wqueens = board.queens & board.white;
  let mut bqueens = board.queens & board.black;
  while wqueens != 0 {
    let index = wqueens.trailing_zeros() as usize;
    mgscore += QUEEN_VALUE + MG_QUEEN_TABLE[index];
    egscore += QUEEN_VALUE + EG_QUEEN_TABLE[index];
    game_phase += queen;
    wqueens &= wqueens - 1;
  }
  while bqueens != 0 {
    let index = bqueens.trailing_zeros() as usize;
    mgscore -= QUEEN_VALUE + MG_QUEEN_TABLE[flip_index(index)];
    egscore -= QUEEN_VALUE + EG_QUEEN_TABLE[flip_index(index)];
    game_phase += queen;
    bqueens &= bqueens - 1;
  }
  let mut wkings = board.kings & board.white;
  let mut bkings = board.kings & board.black;
  while wkings != 0 {
    let index = wkings.trailing_zeros() as usize;
    mgscore += KING_VALUE + MG_KING_TABLE[index];
    egscore += KING_VALUE + EG_KING_TABLE[index];
    wkings &= wkings - 1;
  }
  while bkings != 0 {
    let index = bkings.trailing_zeros() as usize;
    mgscore -= KING_VALUE + MG_KING_TABLE[flip_index(index)];
    egscore -= KING_VALUE + EG_KING_TABLE[flip_index(index)];
    bkings &= bkings - 1;
  }
  if(game_phase > 24) {
    game_phase = 24;
  }
  let egPhase = 24 - game_phase;
  (mgscore * game_phase + egscore * egPhase) / 24
}

pub fn calculate_pawn_structure(board: &Board, egphase: i32) -> i32 {
  let mut score: i32 = 0;
  let mut wpawns: u64 = board.pawns & board.white;
  let mut bpawns: u64 = board.pawns & board.black;
  let multiplier = (MG_PASSED_PAWN * (24-egphase) + EG_PASSED_PAWN * egphase)/24;
  let wpassed_pawns = w_passed_pawn_count(wpawns, bpawns);
  let bpassed_pawns = b_passed_pawn_count(wpawns, bpawns);
  score += wpassed_pawns * multiplier;
  score -= bpassed_pawns * multiplier;
  //TODO: Add more pawn structure evaluation

  score
}

pub fn calculate_king_safety(board: &Board) -> i32 {
  let mut score: i32 = 0;

  score
}

pub fn calculate_mobility(board: &Board) -> i32 {
  let mobility_multiplier: i32 = 5;
  let mut score: i32 = 0;
  score += generate_all_moves(board).len() as i32 * mobility_multiplier;
  //If we decide that board can be allowed to be mutable, then board.is_white_move = !board.is_white_move;
  //score -= generate_all_moves(board).len() as i32 * mobility_multiplier;
  score
}

pub fn calculate_bishop_pair(board: &Board) -> i32 {
  let mut score: i32 = 0;
  let mut wbishops = board.bishops & board.white;
  let mut bbishops = board.bishops & board.black;
  if wbishops.count_ones() > 1 {
    score += 50;
  }
  if bbishops.count_ones() > 1 {
    score -= 50;
  }
  score
}

pub fn rook_on_open_file(board: &Board) -> i32 {
  let mut score: i32 = 0;
  let mut wrooks = board.rooks & board.white;
  let mut brooks = board.rooks & board.black;
  let mut wrooks_on_open_file = wrooks & !file_fill(board.pawns);
  let mut brooks_on_open_file = brooks & !file_fill(board.pawns);
  while wrooks_on_open_file != 0 {
    let index = wrooks_on_open_file.trailing_zeros() as usize;
    score += ROOK_OPEN_FILE;
    wrooks_on_open_file &= wrooks_on_open_file - 1;
  }
  while brooks_on_open_file != 0 {
    let index = brooks_on_open_file.trailing_zeros() as usize;
    score -= ROOK_OPEN_FILE;
    brooks_on_open_file &= brooks_on_open_file - 1;
  }
  score
}

pub fn rook_on_semi_open_file(board: &Board) -> i32 {
  let mut score: i32 = 0;
  let mut wrooks = board.rooks & board.white;
  let mut brooks = board.rooks & board.black;
  let mut wrooks_on_semi_open_file = wrooks & file_fill(board.pawns) & !board.pawns;
  let mut brooks_on_semi_open_file = brooks & file_fill(board.pawns) & !board.pawns;
  while wrooks_on_semi_open_file != 0 {
    let index = wrooks_on_semi_open_file.trailing_zeros() as usize;
    score += ROOK_SEMI_FILE;
    wrooks_on_semi_open_file &= wrooks_on_semi_open_file - 1;
  }
  while brooks_on_semi_open_file != 0 {
    let index = brooks_on_semi_open_file.trailing_zeros() as usize;
    score -= ROOK_SEMI_FILE;
    brooks_on_semi_open_file &= brooks_on_semi_open_file - 1;
  }
  score
}

pub fn ab_pruning(board: &Board, alpha: i32, beta: i32, mve: u32, depth: u32, maximizing_player: bool) -> [i32; 2] {
  if depth == 0 {
    return [evaluate_board(board) as i32, mve as i32];
  }
  let mut moves = generate_all_moves(board);
  if maximizing_player {
    let mut value: i32 = f64::NEG_INFINITY as i32;
    for i in 0..moves.len() {
      let new_board = simulate_move(board, moves[i]);
      let [score,_] = ab_pruning(&new_board, alpha, beta, moves[i], depth - 1, false);
      if score > value {
        value = score;
        let best_move = moves[i];
      }
      alpha = alpha.max(best_score);
      if value >= beta {
        break;
      }

    }
    return [value, best_move]
  } else {
    let mut value = f64::INFINITY as i32;
    for i in 0..moves.len() {
      let new_board = simulate_move(board, moves[i]);
      let [score_] = ab_pruning(&new_board, alpha, beta, moves[i], depth - 1, true)[0];
      if score < value {
        value = score;
        best_move = moves[i];
      }
      beta = beta.min(best_score);
      if value <= alpha {
        break;
      }
    }
  }
}