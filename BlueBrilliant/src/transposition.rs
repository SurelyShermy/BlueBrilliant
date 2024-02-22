use crate::board::*;
use rand::*;
use std::collections::HashMap;

const NUM_PIECES: usize = 12;
const NUM_SQUARES: usize = 64;
const NUM_CASTLING_OPTIONS: usize = 4;
const NUM_EN_PASSANT_FILES: usize = 8;

const WPAWN: usize = 0;
const WKNIGHT: usize = 1;
const WBISHOP: usize = 2;
const WROOK: usize = 3;
const WQUEEN: usize = 4;
const WKING: usize = 5;
const BPAWN: usize = 6;
const BKNIGHT: usize = 7;
const BBISHOP: usize = 8;
const BROOK: usize = 9;
const BQUEEN: usize = 10;
const BKING: usize = 11;

const EXACT: u8 = 0;
const LOWERBOUND: u8 = 1;
const UPPERBOUND: u8 = 2;

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
    pub fn compute_hash(&self, board: &mut Board)->u64{
        let mut hash = 0;
        let mut white_pawns = board.white() & board.pawns();
        let mut white_knights = board.white() & board.knights();
        let mut white_bishops = board.white() & board.bishops();
        let mut white_rooks = board.white() & board.rooks();
        let mut white_queens = board.white() & board.queens();
        let mut white_king = board.white() & board.kings();
        let mut black_pawns = board.black() & board.pawns();
        let mut black_knights = board.black() & board.knights();
        let mut black_bishops = board.black() & board.bishops();
        let mut black_rooks = board.black() & board.rooks();
        let mut black_queens = board.black() & board.queens();
        let mut black_king = board.black() & board.kings();
        while white_pawns != 0 {
            let index = white_pawns.trailing_zeros() as usize;
            hash ^= self.piece_keys[index][WPAWN];
            white_pawns &= white_pawns - 1; 
        }
        while white_knights != 0 {
            let index = white_knights.trailing_zeros() as usize;
            hash ^= self.piece_keys[index][WKNIGHT]; 
            white_knights &= white_knights - 1;
        }
        while white_bishops != 0 {
            let index = white_bishops.trailing_zeros() as usize;
            hash ^= self.piece_keys[index][WBISHOP]; 
            white_bishops &= white_bishops - 1;
        }
        while white_rooks != 0 {
            let index = white_rooks.trailing_zeros() as usize;
            hash ^= self.piece_keys[index][WROOK]; 
            white_rooks &= white_rooks - 1;
        }
        while white_queens != 0 {
            let index = white_queens.trailing_zeros() as usize;
            hash ^= self.piece_keys[index][WQUEEN]; 
            white_queens &= white_queens - 1; 
        }
        while white_king != 0 {
            let index = white_king.trailing_zeros() as usize;
            hash ^= self.piece_keys[index][WKING]; 
            white_king &= white_king - 1; 
        }
        while black_pawns != 0 {
            let index = black_pawns.trailing_zeros() as usize;
            hash ^= self.piece_keys[index][BPAWN]; 
            black_pawns &= black_pawns - 1; 
        }
        while black_knights != 0 {
            let index = black_knights.trailing_zeros() as usize;
            hash ^= self.piece_keys[index][BKNIGHT]; 
            black_knights &= black_knights - 1; 
        }
        while black_bishops != 0 {
            let index = black_bishops.trailing_zeros() as usize;
            hash ^= self.piece_keys[index][BBISHOP];
            black_bishops &= black_bishops - 1; 
        }
        while black_rooks != 0 {
            let index = black_rooks.trailing_zeros() as usize;
            hash ^= self.piece_keys[index][BROOK]; 
            black_rooks &= black_rooks - 1; 
        }
        while black_queens != 0 {
            let index = black_queens.trailing_zeros() as usize;
            hash ^= self.piece_keys[index][BQUEEN];
            black_queens &= black_queens - 1; 
        }
        while black_king != 0 {
            let index = black_king.trailing_zeros() as usize;
            hash ^= self.piece_keys[index][BKING];
            black_king &= black_king - 1; 
        }

        // Castling rights
        if board.white_castle_short() { hash ^= self.castling_keys[0]; }
        if board.white_castle_long() { hash ^= self.castling_keys[1]; }
        if board.black_castle_short() { hash ^= self.castling_keys[2]; }
        if board.black_castle_long() { hash ^= self.castling_keys[3]; }

        // En passant
        if board.en_passant_target() != 0{
            let index: usize = board.en_passant_target().trailing_zeros() as usize;
            let file: usize = index % 8;
            hash ^= self.en_passant_keys[file as usize];
        }
        // Side to move
        if board.is_white_move() {
            hash ^= self.side_key;
        }

        hash
    }
}


pub struct TableEntry{
    key: u64,
    depth: u32,
    best_move: Option<(u8, u8)>,
    score: i32,
    node_type: u8,
}
impl TableEntry{
    pub fn new(key: u64, depth: u32, best_move: Option<(u8, u8)>, score: i32, node_type: u8) -> TableEntry{
        TableEntry{
            key,
            depth,
            best_move,
            score,
            node_type,
        }
    }
    pub fn key(&self) -> u64{
        self.key
    }
    pub fn depth(&self) -> u32{
        self.depth
    }
    pub fn best_move(&self) -> Option<(u8, u8)>{
        self.best_move
    }
    pub fn score(&self) -> i32{
        self.score
    }
    pub fn node_type(&self) -> u8{
        self.node_type
    }
}
pub struct TranspositionTable {
    table: HashMap<u64, TableEntry>,
    size: usize,
}

impl TranspositionTable {
    pub fn new(size: usize) -> Self {
        TranspositionTable {
            table: HashMap::with_capacity(size),
            size,
        }
    }

    pub fn store(&mut self, entry: TableEntry) {
        if self.table.len() >= self.size {
            let key_to_remove = *self.table.keys().next().unwrap();
            self.table.remove(&key_to_remove);
        }
        self.table.insert(entry.key, entry);
    }

    pub fn lookup(&self, hash: u64) -> Option<&TableEntry> {
        self.table.get(&hash)
    }
}
