pub mod board;
use std::io;
use std::mem;

const ALL_SQUARES: u64 = 0xffffffffffffffff;
fn main(){
    let mut board = board::create_board();
    //board::load_fen(&mut board, "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
    println!("What would you like to do m for manual or a number for depth");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    if choice.trim().starts_with("m"){
        loop { 
            let mut start = String::new();
            let mut end = String::new();   
            
            io::stdin().read_line(&mut start).unwrap();
            if start.trim().starts_with("m") {
                board::print_current_moves(&board);           
            } else if start.trim().starts_with("a") {
                board::print_attacks(&board);
            } else if start.trim().starts_with("w") {
                board::print_white(&board);
            } else if start.trim().starts_with("b") {
                board::print_black(&board);
            } else {
                io::stdin().read_line(&mut end).unwrap();
                board::make_move_notation(&mut board, start.clone(), end.clone());
                board::print_board(&board);
            }
        }
    } else {
        let num: i32 = choice.trim().parse().unwrap();
        let mut current: Vec<board::Board> = Vec::new();
        let mut next: Vec<board::Board> = Vec::new();
        let mut count: usize = 0; 
        current.push(board);
        for i in 0..num {
            while let Some(cur_board) = current.pop() {
                next.extend(board::generate_legal_boards(&cur_board));
            }
            count = next.len();
            mem::swap(&mut current, &mut next);
            next.clear();
        }
        println!("Generated {} positions", count); 
    }
}
