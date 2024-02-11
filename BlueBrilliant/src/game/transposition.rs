use board::*;
use rand::*;
use std::collections::HashMap;

const NUM_PIECES: usize = 12; // Assuming 12 types of pieces (6 for each color)
const NUM_SQUARES: usize = 64;
const NUM_CASTLING_OPTIONS: usize = 4;
const NUM_EN_PASSANT_FILES: usize = 8;

pub struct Zobrist{
    piece_keys: [[u64; NUM_PIECES]; NUM_SQUARES],
    castling_keys: [u64; NUM_CASTLING_OPTIONS],
    en_passant_keys: [u64; NUM_EN_PASSANT_FILES],
    side_key: u64,
}
impl Zobrist{
    pub fn new() -> Zobrist{
        let mut rng = rand::thread_rng();
        let piece_keys = [[0; NUM_PIECES]; NUM_SQUARES].map(|piece_row| {
            piece_row.map(|_| rng.gen::<u64>())
        });
        let castling_keys = [0; NUM_CASTLING_OPTIONS].map(|_| rng.gen::<u64>());
        let en_passant_keys = [0; NUM_EN_PASSANT_FILES].map(|_| rng.gen::<u64>());
        let side_key = rng.gen::<u64>();

        Zobrist {
            piece_keys,
            castling_keys,
            en_passant_keys,
            side_key,
        }
    }
    // fn compute_hash(&self, board: Board)->u64{
    //     let mut hash = 0;

    //     for (square, piece) in game_state.board.iter().enumerate() {
    //         if let Some(piece_type) = piece {
    //             hash ^= self.piece_keys[square][*piece_type];
    //         }
    //     }

    //     // Castling rights
    //     if board.white_castle_short() { hash ^= self.castling_keys[0]; }
    //     if board.white_castle_long() { hash ^= self.castling_keys[1]; }
    //     if board.black_castle_short() { hash ^= self.castling_keys[2]; }
    //     if board.black_castle_long() { hash ^= self.castling_keys[3]; }

    //     // En passant
    //     if let Some(file) = (board.en_passant_target() & 7) % 8 {
    //         hash ^= self.en_passant_keys[file];
    //     }

    //     // Side to move
    //     if board.side_to_move() {
    //         hash ^= self.side_key;
    //     }

    //     hash
    // }
}


struct table_entry{
    key: u64,
    depth: u8,
    best_move: Option<(u8, u8)>,
    score: i16,
    node_type: u8,
}

pub struct TranspositionTable {
    table: HashMap<u64, table_entry>,
    size: usize,
}

impl TranspositionTable {
    pub fn new(size: usize) -> Self {
        TranspositionTable {
            table: HashMap::with_capacity(size),
            size,
        }
    }

    // fn store(&mut self, entry: table_entry) {
    //     if self.table.len() >= self.size {
    //         let key_to_remove = *self.table.keys().next().unwrap();
    //         self.table.remove(&key_to_remove);
    //     }
    //     self.table.insert(entry.hash, entry);
    // }

    // fn lookup(&self, hash: u64) -> Option<&table_entry> {
    //     self.table.get(&hash)
    // }
}