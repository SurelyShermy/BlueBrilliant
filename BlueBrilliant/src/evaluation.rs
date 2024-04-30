
use crate::board::*;
use crate::transposition::*;

use std::time::{Duration, Instant};


//Piece Square tables
fn flip_index(index: usize) -> usize {
    index ^ 56
}

const MG_PAWN_VALUE: i32 = 82;
const MG_KNIGHT_VALUE: i32 = 337;
const MG_BISHOP_VALUE: i32 = 365;
const MG_ROOK_VALUE: i32 = 477;
const MG_QUEEN_VALUE: i32 = 1025;
const MG_KING_VALUE: i32 = 5000;

const EG_PAWN_VALUE: i32 = 94;
const EG_KNIGHT_VALUE: i32 = 281;
const EG_BISHOP_VALUE: i32 = 297;
const EG_ROOK_VALUE: i32 = 512;
const EG_QUEEN_VALUE: i32 = 936;
const EG_KING_VALUE: i32 = 5000;

const MAX_QUIESCENCE_DEPTH: u32 = 5;
const EXACT: u8 = 0;
const LOWERBOUND: u8 = 1;
const UPPERBOUND: u8 = 2;
const MG_PASSED_PAWN: i32 = 25;
const EG_PASSED_PAWN: i32 = 50;
const BROKEN_PAWN_SHELTER: i32 = -50;
const ROOK_OPEN_FILE: i32 = 30;
const ROOK_SEMI_FILE: i32 = 5;
const NULL_MOVE: (u8,u8) = (0,0);
const BLACK_MG_PAWN_TABLE: [i32; 64] = [
     0,   0,   0,   0,   0,   0,  0,   0,
   180, 216, 143, 177, 150, 208, 116,  71,
    76,  89, 108, 113, 147, 138, 107,  62,
    68,  95,  88, 103, 105,  94,  99,  59,
    55,  80,  77,  94,  99,  88,  92,  57,
    56,  78,  78,  72,  85,  85, 115,  70,
    47,  81,  62,  59,  67, 106, 120,  60,
     0,   0,   0,   0,   0,   0,  0,   0,
];
const WHITE_MG_PAWN_TABLE: [i32; 64] = [
     0,   0,   0,   0,   0,   0,  0,   0,
    47,  81,  62,  59,  67, 106, 120,  60,
    56,  78,  78,  72,  85,  85, 115,  70,
    55,  80,  77,  94,  99,  88,  92,  57,
    68,  95,  88, 103, 105,  94,  99,  59,
    76,  89, 108, 113, 147, 138, 107,  62,
   180, 216, 143, 177, 150, 208, 116,  71,
     0,   0,   0,   0,   0,   0,  0,   0,
];
const BLACK_EG_PAWN_TABLE: [i32; 64] = [
     0,   0,   0,   0,   0,   0,   0,   0,
   272, 267, 252, 228, 241, 226, 259, 281,
   188, 194, 179, 161, 150, 147, 176, 178,
   126, 118, 107,  99,  92,  98, 111, 111,
   107, 103,  91,  87,  87,  86,  97,  93,
    98, 101,  88,  95,  94,  89,  93,  86,
   107, 102, 102, 104, 107,  94,  96,  87,
     0,   0,   0,   0,   0,   0,   0,   0,
];
const WHITE_EG_PAWN_TABLE: [i32; 64] = [
     0,   0,   0,   0,   0,   0,   0,   0,
   107, 102, 102, 104, 107,  94,  96,  87,
    98, 101,  88,  95,  94,  89,  93,  86,
   107, 103,  91,  87,  87,  86,  97,  93,
   126, 118, 107,  99,  92,  98, 111, 111,
   188, 194, 179, 161, 150, 147, 176, 178,
   272, 267, 252, 228, 241, 226, 259, 281,
     0,   0,   0,   0,   0,   0,   0,   0,
];
const BLACK_MG_KNIGHT_TABLE: [i32; 64] = [
    170, 248, 303, 288, 398, 240, 322, 230,
    264, 296, 409, 373, 360, 399, 344, 320,
    290, 397, 374, 402, 421, 466, 410, 381,
    328, 354, 356, 390, 374, 406, 355, 359,
    324, 341, 353, 350, 365, 356, 358, 329,
    314, 328, 349, 347, 356, 354, 362, 321,
    308, 284, 325, 334, 336, 355, 323, 318,
    232, 316, 279, 304, 320, 309, 318, 314
];
const WHITE_MG_KNIGHT_TABLE: [i32; 64] = [
    232, 316, 279, 304, 320, 309, 318, 314,
    308, 284, 325, 334, 336, 355, 323, 318,
    314, 328, 349, 347, 356, 354, 362, 321,
    324, 341, 353, 350, 365, 356, 358, 329,
    328, 354, 356, 390, 374, 406, 355, 359,
    290, 397, 374, 402, 421, 466, 410, 381,
    264, 296, 409, 373, 360, 399, 344, 320,
    170, 248, 303, 288, 398, 240, 322, 230
];

const BLACK_EG_KNIGHT_TABLE: [i32; 64] = [
    223, 243, 268, 253, 250, 254, 218, 182,
    256, 273, 256, 279, 272, 256, 257, 229,
    257, 261, 291, 290, 280, 272, 262, 240,
    264, 284, 303, 303, 303, 292, 289, 263,
    263, 275, 297, 306, 297, 298, 285, 263,
    258, 278, 280, 296, 291, 278, 261, 259,
    239, 261, 271, 276, 279, 261, 258, 237,
    252, 230, 258, 266, 259, 263, 231, 217,
];
const WHITE_EG_KNIGHT_TABLE: [i32; 64] = [
    252, 230, 258, 266, 259, 263, 231, 217,
    239, 261, 271, 276, 279, 261, 258, 237,
    258, 278, 280, 296, 291, 278, 261, 259,
    263, 275, 297, 306, 297, 298, 285, 263,
    264, 284, 303, 303, 303, 292, 289, 263,
    257, 261, 291, 290, 280, 272, 262, 240,
    256, 273, 256, 279, 272, 256, 257, 229,
    223, 243, 268, 253, 250, 254, 218, 182,
];

const BLACK_MG_BISHOP_TABLE: [i32; 64] = [
  336, 369, 283, 328, 340, 323, 372, 357,
  339, 381, 347, 352, 395, 424, 383, 318,
  349, 402, 408, 405, 400, 415, 402, 363,
  361, 370, 384, 415, 402, 402, 372, 363,
  359, 378, 378, 391, 399, 377, 375, 369,
  365, 380, 380, 380, 379, 392, 383, 375,
  369, 380, 381, 365, 372, 386, 398, 366,
  332, 362, 351, 344, 352, 353, 326, 344,
];
const WHITE_MG_BISHOP_TABLE: [i32; 64] = [
  332, 362, 351, 344, 352, 353, 326, 344,
  369, 380, 381, 365, 372, 386, 398, 366,
  365, 380, 380, 380, 379, 392, 383, 375,
  359, 378, 378, 391, 399, 377, 375, 369,
  361, 370, 384, 415, 402, 402, 372, 363,
  349, 402, 408, 405, 400, 415, 402, 363,
  339, 381, 347, 352, 395, 424, 383, 318,
  336, 369, 283, 328, 340, 323, 372, 357,
];
const BLACK_EG_BISHOP_TABLE: [i32; 64] = [
    283, 276, 286, 289, 290, 288, 280, 273,
    289, 293, 304, 285, 294, 284, 293, 283,
    299, 289, 297, 296, 295, 303, 297, 301,
    294, 306, 309, 306, 311, 307, 300, 299,
    291, 300, 310, 316, 304, 307, 294, 288,
    285, 294, 305, 307, 310, 300, 290, 282,
    283, 279, 290, 296, 301, 288, 282, 270,
    274, 288, 274, 292, 288, 281, 292, 280,
];
const WHITE_EG_BISHOP_TABLE: [i32; 64] = [
    274, 288, 274, 292, 288, 281, 292, 280,
    283, 279, 290, 296, 301, 288, 282, 270,
    285, 294, 305, 307, 310, 300, 290, 282,
    291, 300, 310, 316, 304, 307, 294, 288,
    294, 306, 309, 306, 311, 307, 300, 299,
    299, 289, 297, 296, 295, 303, 297, 301,
    289, 293, 304, 285, 294, 284, 293, 283,
    283, 276, 286, 289, 290, 288, 280, 273,
];
const BLACK_MG_ROOK_TABLE: [i32; 64] = [
    509, 519, 509, 528, 540, 486, 508, 520,
    504, 509, 535, 539, 557, 544, 503, 521,
    472, 496, 503, 513, 494, 522, 538, 493,
    453, 466, 484, 503, 501, 512, 469, 457,
    441, 451, 465, 476, 486, 470, 483, 454,
    432, 452, 461, 460, 480, 477, 472, 444,
    433, 461, 457, 468, 476, 488, 471, 406,
    458, 464, 478, 494, 493, 484, 440, 451,
];
const WHITE_MG_ROOK_TABLE: [i32; 64] = [
    458, 464, 478, 494, 493, 484, 440, 451,
    433, 461, 457, 468, 476, 488, 471, 406,
    432, 452, 461, 460, 480, 477, 472, 444,
    441, 451, 465, 476, 486, 470, 483, 454,
    453, 466, 484, 503, 501, 512, 469, 457,
    472, 496, 503, 513, 494, 522, 538, 493,
    504, 509, 535, 539, 557, 544, 503, 521,
    509, 519, 509, 528, 540, 486, 508, 520,
];
const BLACK_EG_ROOK_TABLE: [i32; 64] = [
    525, 522, 530, 527, 524, 524, 520, 517,
    523, 525, 525, 523, 509, 515, 520, 515,
    519, 519, 519, 517, 516, 509, 507, 509,
    516, 515, 525, 513, 514, 513, 511, 514,
    515, 517, 520, 516, 507, 506, 504, 501,
    508, 512, 507, 511, 505, 500, 504, 496,
    506, 506, 512, 514, 503, 503, 501, 509,
    503, 514, 515, 511, 507, 499, 516, 492,
];
const WHITE_EG_ROOK_TABLE: [i32; 64] = [
    503, 514, 515, 511, 507, 499, 516, 492,
    506, 506, 512, 514, 503, 503, 501, 509,
    508, 512, 507, 511, 505, 500, 504, 496,
    515, 517, 520, 516, 507, 506, 504, 501,
    516, 515, 525, 513, 514, 513, 511, 514,
    519, 519, 519, 517, 516, 509, 507, 509,
    523, 525, 525, 523, 509, 515, 520, 515,
    525, 522, 530, 527, 524, 524, 520, 517,
];
const BLACK_MG_QUEEN_TABLE: [i32; 64] = [
997, 1025, 1054, 1037, 1084, 1069, 1068, 1070,
1001, 986, 1020, 1026, 1009, 1082, 1053, 1079,
1012, 1008, 1032, 1033, 1054, 1081, 1072, 1082,
998, 998, 1009, 1009, 1024, 1042, 1023, 1026,
1016, 999, 1016, 1015, 1023, 1021, 1028, 1022,
1011, 1027, 1014, 1023, 1020, 1027, 1039, 1030,
990, 1017, 1036, 1027, 1033, 1040, 1022, 1026,
1024, 1007, 1016, 1035, 1010, 1000, 994, 975
];
const WHITE_MG_QUEEN_TABLE: [i32; 64] = [
  1024, 1007, 1016, 1035, 1010, 1000, 994, 975,
  990, 1017, 1036, 1027, 1033, 1040, 1022, 1026,
  1011, 1027, 1014, 1023, 1020, 1027, 1039, 1030,
  1016, 999, 1016, 1015, 1023, 1021, 1028, 1022,
  998, 998, 1009, 1009, 1024, 1042, 1023, 1026,
  1012, 1008, 1032, 1033, 1054, 1081, 1072, 1082,
  1001, 986, 1020, 1026, 1009, 1082, 1053, 1079,
  997, 1025, 1054, 1037, 1084, 1069, 1068, 1070
];
const BLACK_EG_QUEEN_TABLE: [i32; 64] = [
  927, 958, 958, 963, 963, 955, 946, 956,
  919, 956, 968, 977, 994, 961, 966, 936,
  916, 942, 945, 985, 983, 971, 955, 945,
  939, 958, 960, 981, 993, 976, 993, 972,
  918, 964, 955, 983, 967, 970, 975, 959,
  920, 909, 951, 942, 945, 953, 946, 941,
  914, 913, 906, 920, 920, 913, 900, 904,
  903, 908, 914, 893, 931, 904, 916, 895,
];

const WHITE_EG_QUEEN_TABLE: [i32; 64] = [
  903, 908, 914, 893, 931, 904, 916, 895,
  914, 913, 906, 920, 920, 913, 900, 904,
  920, 909, 951, 942, 945, 953, 946, 941,
  918, 964, 955, 983, 967, 970, 975, 959,
  939, 958, 960, 981, 993, 976, 993, 972,
  916, 942, 945, 985, 983, 971, 955, 945,
  919, 956, 968, 977, 994, 961, 966, 936,
  927, 958, 958, 963, 963, 955, 946, 956,
];

const BLACK_MG_KING_TABLE: [i32; 64] = [
    4935, 5023, 5016, 4985, 4944, 4966, 5002, 5013,
    5029, 4999, 4980, 4993, 4992, 4996, 4962, 4971,
    4991, 5024, 5002, 4984, 4980, 5006, 5022, 4978,
    4983, 4980, 4988, 4973, 4970, 4975, 4986, 4964,
    4951, 4999, 4973, 4961, 4954, 4956, 4967, 4949,
    4986, 4986, 4978, 4954, 4956, 4970, 4985, 4973,
    5001, 5007, 4992, 4936, 4957, 4984, 5009, 5008,
    4985, 5036, 5012, 4946, 5008, 4972, 5024, 5014,
];

const WHITE_MG_KING_TABLE: [i32; 64] = [
    4985, 5036, 5012, 4946, 5008, 4972, 5024, 5014,
    5001, 5007, 4992, 4936, 4957, 4984, 5009, 5008,
    4986, 4986, 4978, 4954, 4956, 4970, 4985, 4973,
    4951, 4999, 4973, 4961, 4954, 4956, 4967, 4949,
    4983, 4980, 4988, 4973, 4970, 4975, 4986, 4964,
    4991, 5024, 5002, 4984, 4980, 5006, 5022, 4978,
    5029, 4999, 4980, 4993, 4992, 4996, 4962, 4971,
    4935, 5023, 5016, 4985, 4944, 4966, 5002, 5013,
];
const BLACK_EG_KING_TABLE: [i32; 64] = [
  4926, 4965, 4982, 4982, 4989, 5015, 5004, 4983,
  4988, 5017, 5014, 5017, 5017, 5038, 5023, 5011,
  5010, 5017, 5023, 5015, 5020, 5045, 5044, 5013,
  4992, 5022, 5024, 5027, 5026, 5033, 5026, 5003,
  4982, 4996, 5021, 5024, 5027, 5023, 5009, 4989,
  4981, 4997, 5011, 5021, 5023, 5016, 5007, 4991,
  4973, 4989, 5004, 5013, 5014, 5004, 4995, 4983,
  4947, 4966, 4979, 4989, 4972, 4986, 4976, 4957,
];
const WHITE_EG_KING_TABLE: [i32; 64] = [
  4947, 4966, 4979, 4989, 4972, 4986, 4976, 4957,
  4973, 4989, 5004, 5013, 5014, 5004, 4995, 4983,
  4981, 4997, 5011, 5021, 5023, 5016, 5007, 4991,
  4982, 4996, 5021, 5024, 5027, 5023, 5009, 4989,
  4992, 5022, 5024, 5027, 5026, 5033, 5026, 5003,
  5010, 5017, 5023, 5015, 5020, 5045, 5044, 5013,
  4988, 5017, 5014, 5017, 5017, 5038, 5023, 5011,
  4926, 4965, 4982, 4982, 4989, 5015, 5004, 4983,
];

//File utility functions from https://www.chessprogramming.org/Pawn_Fills
pub fn north_fill(pawns: u64) -> u64 {
  let mut pawns = pawns;
  pawns |= pawns << 8;
  pawns |= pawns << 16;
  pawns |= pawns << 32;
  pawns
}
pub struct Evaluation {
  transposition_table: TranspositionTable,
  zobrist_keys: Zobrist,
  pub exact_match: u64,
  pub upper_match: u64,
  pub lower_match: u64, 
  pub raw_match: u64,
}


impl Evaluation{
  pub fn new() -> Self {
    Evaluation {
        transposition_table: TranspositionTable::new(100000000),
        zobrist_keys: Zobrist::new(),
        exact_match: 0,
        upper_match: 0,
        lower_match: 0,
        raw_match: 0,
    }
  }
  pub fn iterative_deepening_ab_pruning(&mut self, board: &mut Board, initial_alpha: i32, initial_beta: i32, mve: (u8, u8), max_depth: u32, maximizing_player: bool) -> (i32, (u8, u8), u32) {
    let mut best_move = mve;
    let mut best_score = if maximizing_player { i32::MIN } else { i32::MAX };
    let mut total_node_count = 0;
    println!("Sanity Check starting eval before loop {}", evaluate_board(board));
    println!("max depth: {}", max_depth);
    let start = Instant::now();
    for depth in 1..=max_depth {
        if(start.elapsed().as_secs() > 30){
          print!("Final depth was {}\n", depth);
          return (best_score, best_move, total_node_count);
        }
        let (score, move_at_depth, node_count) = self.ab_pruning(board, initial_alpha, initial_beta, best_move, depth, maximizing_player, start, max_depth, 0);
        total_node_count += node_count;

        if (depth > 4 && ((maximizing_player && (score > best_score)) || (!maximizing_player && (score < best_score)))) {
            println!("Depth: {}, Score: {}, Move: {:?}", depth, score, move_at_depth);
            
            best_move = move_at_depth;
            best_score = score;
            if (best_move) == NULL_MOVE{
              println!("The depth this occured at was {}", depth);
              println!("This occured at node count {}", total_node_count);
              println!("max depth is {}", max_depth);
              println!("this is a bad return in iter deepening");
            }
        }
    }
    if (best_move) == NULL_MOVE{
      println!("this is a bad return in iter deepening outside of loop");
    }
    (best_score, best_move, total_node_count)
  }
  fn quiescence_search(&mut self, board: &mut Board, alpha: i32, beta: i32, node_count: &mut u32, depth: u32) -> i32 {
    *node_count += 1;
    let mut alpha = alpha;
    let stand_pat = evaluate_board(board);
    if depth == MAX_QUIESCENCE_DEPTH {
      return stand_pat;
    }
    if stand_pat >= beta {
        return beta;
    }
    if alpha < stand_pat {
        alpha = stand_pat;
    }
    

    let capture_moves = capture_moves_only(board);
    for i in (0..capture_moves.len()).step_by(2) {
        if is_promotion(board, capture_moves[i]){
          if board.is_white_move(){
            let direction = (capture_moves[i+1]-capture_moves[i]-7)<<6;
            for j in 0..4{
              let end = direction | j<<4;
              let mut new_board: Board = simulate_move(board, capture_moves[i], end);

              let score = -self.quiescence_search(&mut new_board, -beta, -alpha, node_count, depth+1);
              if score >= beta {
                return beta;
              }
              if score > alpha {
                  alpha = score;
              }
            }
          }
          else{
            let direction = ((capture_moves[i+1]+9)-capture_moves[i])<<6;
            for j in 0..4{
              let end = direction | j<<4;
              let mut new_board: Board = simulate_move(board, capture_moves[i], end);
              let score = -self.quiescence_search(&mut new_board, -beta, -alpha, node_count, depth+1);
              if score >= beta {
                return beta;
              }
              if score > alpha {
                  alpha = score;
              }
            }
          }
        }
        else{
          let mut new_board: Board = simulate_move(board, capture_moves[i], capture_moves[i + 1]);
          let score = -self.quiescence_search(&mut new_board, -beta, -alpha, node_count, depth+1);
          if score >= beta {
            return beta;
          }
          if score > alpha {
              alpha = score;
          }
        }
    }

    alpha
}
  pub fn ab_pruning(&mut self, board: &mut Board, initial_alpha: i32, initial_beta: i32, mve: (u8, u8), depth: u32, maximizing_player: bool, time: Instant, max_depth: u32, ply: u32) -> (i32, (u8, u8), u32) {
    let mut node_count = 1;
    
    let hash = self.zobrist_keys.compute_hash(board);
    let ttval = self.transposition_table.lookup(hash);
    if can_claim_draw(board, hash){
      return (0, mve, node_count);
    }
    // //Pass by reference instead?
    let mut alpha = initial_alpha;
    let mut beta = initial_beta;
    match ttval{
      Some(x) => 'block: {
        // println!("Found in TT");
        if x.open(){
          if x.best_move().unwrap() == NULL_MOVE{
            println!("this is a bad return in open cutoff");
          }
          x.set_open(false);
          return (0, x.best_move().unwrap(), node_count);
        }
        x.set_open(true);
        self.raw_match += 1;
        //If the depth that we are searching is greater than or equal to the depth of the stored position in the transposition table
        if x.depth() as u32 >= (depth) && x.depth() as u32 >= 4 {
          if x.node_type() == EXACT {
            self.exact_match += 1;
            if x.best_move().unwrap() == NULL_MOVE{
              println!("this is a bad return in exact");
            }
            return (x.score(), x.best_move().unwrap(), node_count);
          } else if x.node_type() == LOWERBOUND {
            self.lower_match += 1;
            alpha = initial_alpha.max(x.score());
          } else if x.node_type() == UPPERBOUND {
            self.upper_match += 1;
            beta = initial_beta.min(x.score());
          }
          if maximizing_player{
            if alpha >= beta {
              x.set_open(false);
              if x.best_move().unwrap() == NULL_MOVE{
                println!("this is a bad return in ab cut off");
              }
              return (x.score(), x.best_move().unwrap(), node_count);
            }
          }else{
            if beta <= alpha {
              x.set_open(false);
              if x.best_move().unwrap() == NULL_MOVE{
                println!("this is a bad return in ab cut off");
              }
              return (x.score(), x.best_move().unwrap(), node_count);
            }
          }
          
        }
      }
      None => {
        // //setting to true since this position has not been reached 
        // let new_entry: TableEntry = TableEntry::new(hash, depth, Some(mve), 0, EXACT, true, true);
        // self.transposition_table.store(new_entry);
      }
    }
    if(time.elapsed().as_secs() > 30){
      let eval = evaluate_board(board);
      self.transposition_table.replace(hash, depth, Some(mve), eval, EXACT, false, false);
      if mve == NULL_MOVE{
        println!("this is a bad return in time elasped");
      }
      return (eval, mve, node_count);
    }
    if depth == 0 {
      let eval = self.quiescence_search(board, alpha, beta, &mut node_count, 0);

      self.transposition_table.replace(hash, depth, Some(mve), eval, EXACT, false, false);
      if mve == NULL_MOVE{
        println!("this is a bad return in depth = 0");
      }
      return (eval, mve, node_count);
    }
    let moves = ab_move_generation(board);
    if moves.len() == 0 {
        if is_check(board) {
            if maximizing_player {
                //Should node type here be exact??
                self.transposition_table.replace(hash, depth, Some(mve), i32::MAX - depth as i32, LOWERBOUND, false, false);
                // let new_entry = TableEntry::new(self.zobrist_keys.compute_hash(board), depth, Some(mve), i32::MIN + depth as i32, LOWERBOUND, false);
                return (i32::MIN + depth as i32, mve, node_count);
            } else {
                self.transposition_table.replace(hash, depth, Some(mve), i32::MIN + depth as i32, UPPERBOUND, false, false);
                // let new_entry = TableEntry::new(self.zobrist_keys.compute_hash(board), depth, Some(mve), i32::MAX - depth as i32, UPPERBOUND, false);
                return (i32::MAX - depth as i32, mve, node_count);
            }
        } else {
            return (0, mve, node_count);
        }
    }
    
    let mut best_move = mve;
    if maximizing_player {
        let mut value = i32::MIN;
  
        for i in (0..moves.len()).step_by(2) {
            if (moves[i], moves[i+1]) == NULL_MOVE{
              println!("this is a move gen error");
            }
            if is_promotion(board, moves[i]){
              if board.is_white_move(){
                let direction = (moves[i+1]-moves[i]-7)<<6;
                for j in 0..4{
                  let end = direction | j<<4;
                  let mut new_board: Board = simulate_move(board, moves[i], end);
                  *new_board.position_counts().entry(hash).or_insert(0)+=1;
                  let (score, _, child_node_count) = Self::ab_pruning(self, &mut new_board, alpha, beta, (moves[i], end), depth - 1, false, time, max_depth, ply+1);
                  node_count += child_node_count;
                  if score > value {
                      value = score;
                      best_move = (moves[i], end);
                  }
                  alpha = alpha.max(value);
                  if value >= initial_beta {
                      break;
                  }
                }
              }
              else{
                let direction = ((moves[i+1]+9)-moves[i])<<6;
                for j in 0..4{
                  let end = direction | j<<4;
                  let mut new_board: Board = simulate_move(board, moves[i], end);
                  *new_board.position_counts().entry(hash).or_insert(0)+=1;
                  let (score, _, child_node_count) = Self::ab_pruning(self, &mut new_board, alpha, beta, (moves[i], end), depth - 1, false, time, max_depth, ply+1);
                  node_count += child_node_count;
                  if score > value {
                      value = score;
                      best_move = (moves[i], end);
                  }
                  alpha = alpha.max(value);
                  if value >= initial_beta {
                      break;
                  }
                }
              }
            }
            else{
              let mut new_board: Board = simulate_move(board, moves[i], moves[i + 1]);
              *new_board.position_counts().entry(hash).or_insert(0)+=1;
              let (score, _, child_node_count) = Self::ab_pruning(self, &mut new_board, alpha, beta, (moves[i], moves[i + 1]), depth - 1, false, time, max_depth, ply+1);
              node_count += child_node_count;
              if score > value {
                  value = score;
                  best_move = (moves[i], moves[i + 1]);
              }
              alpha = alpha.max(value);
              if value >= initial_beta {
                  break;
              }
            }
        }
        let node_type = if value <= initial_alpha {
            LOWERBOUND
        } else if value >= initial_beta {
            UPPERBOUND
        } else {
            EXACT
        };
        self.transposition_table.replace(hash, depth, Some(best_move), value, node_type, false, false);
        // let new_entry = TableEntry::new(self.zobrist_keys.compute_hash(board), depth, Some(best_move), value, node_type, false);
        if best_move == NULL_MOVE{
          println!("this is a return error");
        }        
        (value, best_move, node_count)
    } else {

        let mut value = i32::MAX;
  
        for i in (0..moves.len()).step_by(2) {
          if (moves[i], moves[i+1]) == NULL_MOVE{
            println!("this is a move gen error");
          }
          if is_promotion(board, moves[i]){
            if board.is_white_move(){
              let direction = (moves[i+1]-moves[i]-7)<<6;
              for j in 0..4{
                let end = direction | j<<4;
                let mut new_board: Board = simulate_move(board, moves[i], end);
                *new_board.position_counts().entry(hash).or_insert(0)+=1;
                let (score, _, child_node_count) = Self::ab_pruning(self, &mut new_board, alpha, beta, (moves[i], end), depth - 1, true, time, max_depth, ply+1);
                node_count += child_node_count;
                if score < value {
                  value = score;
                  best_move = (moves[i], moves[i + 1]);
                }
                beta = beta.min(value);
                if value <= initial_alpha {
                    break;
                }
              }
            }
            else{
              let direction = ((moves[i+1]+9)-moves[i])<<6;
              for j in 0..4{
                let end = direction | j<<4;
                let mut new_board: Board = simulate_move(board, moves[i], end);
                *new_board.position_counts().entry(hash).or_insert(0)+=1;
                let (score, _, child_node_count) = Self::ab_pruning(self, &mut new_board, alpha, beta, (moves[i], end), depth - 1, true, time, max_depth, ply+1);
                node_count += child_node_count;
                if score < value {
                  value = score;
                  best_move = (moves[i], moves[i + 1]);
                }
                beta = beta.min(value);
                if value <= initial_alpha {
                    break;
                }
              }
            }
          }else{
            let mut new_board = simulate_move(board, moves[i], moves[i + 1]);
            *new_board.position_counts().entry(hash).or_insert(0)+=1;
            let (score, _, child_node_count) = Self::ab_pruning(self, &mut new_board, alpha, beta, (moves[i], moves[i + 1]), depth - 1, true, time, max_depth, ply+1);
            node_count += child_node_count;
            if score < value {
                value = score;
                best_move = (moves[i], moves[i + 1]);
            }
            beta = beta.min(value);
            if value <= initial_alpha {
                break;
            }
          }
        }
        let node_type = if value <= initial_alpha {
          LOWERBOUND
        } else if value >= initial_beta {
          UPPERBOUND
        } else {
          EXACT
        };
        self.transposition_table.replace(hash, depth, Some(best_move), value, node_type, false, false);
        if best_move == NULL_MOVE{
          println!("this is a return error");
        }
        (value, best_move, node_count)
    }
  } 
}
pub fn evaluate_board(board: & mut Board) -> i32 {
  let mut score: i32 = 0;
  let mut egPhase: i32 = 0;
  //Perhaps calculate material can also pass the eg phase to allow for phase tapering

  [score, egPhase] = calculate_material(board);
  // score += calculate_pawn_structure(board, egPhase);
  // score += calculate_king_safety(board);
  // score += calculate_bishop_pair(board);

  // score += rook_on_open_file(board);
  // score += rook_on_semi_open_file(board);

  score
}
pub fn south_fill(pawns: u64)-> u64 {
  let mut pawns = pawns;
  pawns |= pawns >> 8;
  pawns |= pawns >> 16;
  pawns |= pawns >> 32;
  pawns
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

pub fn calculate_material(board: &Board) -> [i32; 2] {
  //These numbers are based on pesto game phase increment count
  let knight = 1;
  let bishop = 1;
  let rook = 2;
  let queen = 4;

  let mut mgscore: i32 = 0;
  let mut egscore: i32 = 0;
  let mut game_phase: i32 = 0;
  let mut wpawns = board.pawns() & board.white();
  let mut bpawns = board.pawns() & board.black();
  while wpawns != 0 {
    let index = wpawns.trailing_zeros() as usize;
    mgscore += WHITE_MG_PAWN_TABLE[index];
    egscore += WHITE_EG_PAWN_TABLE[index];
    wpawns &= wpawns - 1;
  }
  while bpawns != 0 {
    let index = bpawns.trailing_zeros() as usize;
    mgscore -= BLACK_MG_PAWN_TABLE[index];
    egscore -= BLACK_EG_PAWN_TABLE[index];
    bpawns &= bpawns - 1;
  }
  let mut wknights = board.knights() & board.white();
  let mut bknights = board.knights() & board.black();
  while wknights != 0 {
    let index = wknights.trailing_zeros() as usize;
    mgscore += WHITE_MG_KNIGHT_TABLE[index];
    egscore += WHITE_EG_KNIGHT_TABLE[index];
    game_phase += knight;
    wknights &= wknights - 1;
  }
  while bknights != 0 {
    let index = bknights.trailing_zeros() as usize;
    mgscore -= BLACK_MG_KNIGHT_TABLE[index];
    egscore -= BLACK_EG_KNIGHT_TABLE[index];
    game_phase += knight;
    bknights &= bknights - 1;
  }
  let mut wbishops = board.bishops() & board.white();
  let mut bbishops = board.bishops() & board.black();
  while wbishops != 0 {
    let index = wbishops.trailing_zeros() as usize;
    mgscore += WHITE_MG_BISHOP_TABLE[index];
    egscore += WHITE_EG_BISHOP_TABLE[index];
    game_phase += bishop;
    wbishops &= wbishops - 1;
  }
  while bbishops != 0 {
    let index = bbishops.trailing_zeros() as usize;
    mgscore -= BLACK_MG_BISHOP_TABLE[index];
    egscore -= BLACK_EG_BISHOP_TABLE[index];
    game_phase += bishop;
    bbishops &= bbishops - 1;
  }
  let mut wrooks = board.rooks() & board.white();
  let mut brooks = board.rooks() & board.black();
  while wrooks != 0 {
    let index = wrooks.trailing_zeros() as usize;
    mgscore += WHITE_MG_ROOK_TABLE[index];
    egscore += WHITE_EG_ROOK_TABLE[index];
    game_phase += rook;
    wrooks &= wrooks - 1;
  }
  while brooks != 0 {
    let index = brooks.trailing_zeros() as usize;
    mgscore -= BLACK_MG_ROOK_TABLE[index];
    egscore -= BLACK_EG_ROOK_TABLE[index];
    game_phase += rook;
    brooks &= brooks - 1;
  }
  let mut wqueens = board.queens() & board.white();
  let mut bqueens = board.queens() & board.black();
  while wqueens != 0 {
    let index = wqueens.trailing_zeros() as usize;
    mgscore += WHITE_MG_QUEEN_TABLE[index];
    egscore += WHITE_EG_QUEEN_TABLE[index];
    game_phase += queen;
    wqueens &= wqueens - 1;
  }
  while bqueens != 0 {
    let index = bqueens.trailing_zeros() as usize;
    mgscore -= BLACK_MG_QUEEN_TABLE[index];
    egscore -= BLACK_EG_QUEEN_TABLE[index];
    game_phase += queen;
    bqueens &= bqueens - 1;
  }
  let mut wkings = board.kings() & board.white();
  let mut bkings = board.kings() & board.black();
  while wkings != 0 {
    let index = wkings.trailing_zeros() as usize;
    mgscore += WHITE_MG_KING_TABLE[index];
    egscore += WHITE_EG_KING_TABLE[index];
    wkings &= wkings - 1;
  }
  while bkings != 0 {
    let index = bkings.trailing_zeros() as usize;
    mgscore -= BLACK_MG_KING_TABLE[index];
    egscore -= BLACK_EG_KING_TABLE[index];
    bkings &= bkings - 1;
  }
  if game_phase > 24 {
    game_phase = 24;
  }
  let egPhase = 24 - game_phase;
  [(mgscore * game_phase + egscore * egPhase) / 24, egPhase]
  
}

pub fn calculate_pawn_structure(board: &Board, egphase: i32) -> i32 {
  let mut score: i32 = 0;
  let mut wpawns: u64 = board.pawns() & board.white();
  let mut bpawns: u64 = board.pawns() & board.black();
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


pub fn calculate_bishop_pair(board: &Board) -> i32 {
  let mut score: i32 = 0;
  let mut wbishops = board.bishops() & board.white();
  let mut bbishops = board.bishops() & board.black();
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
  let mut wrooks = board.rooks() & board.white();
  let mut brooks = board.rooks() & board.black();
  let mut wrooks_on_open_file = wrooks & !file_fill(board.pawns());
  let mut brooks_on_open_file = brooks & !file_fill(board.pawns());
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
  let mut wrooks = board.rooks() & board.white();
  let mut brooks = board.rooks() & board.black();
  let mut wrooks_on_semi_open_file = wrooks & file_fill(board.pawns()) & !board.pawns();
  let mut brooks_on_semi_open_file = brooks & file_fill(board.pawns()) & !board.pawns();
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
