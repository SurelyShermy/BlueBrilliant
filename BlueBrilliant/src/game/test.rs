pub mod board;
//pub mod evaluation;
use std::env;
use std::io;
use std::mem;

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() == 4 {
        test_mode_moves(&args[1], &args[2], &args[3]); 
    } else if args.len() == 3 {
        test_mode(&args[1], &args[2]);
    } else if args.len() == 2 {
  //      evaluation_test(&args[1]);
    } else {
        user_mode();
    }

  
}
fn evaluation_test(fen: &String) {
    let mut board = board::create_board();
    board::load_fen(&mut board, &fen);
//    println!("Eval: {}", evaluation::evaluate_board(&board));
}
fn test_mode(depth: &String, fen: &String){
    let mut board = board::create_board();
    board::load_fen(&mut board, &fen);
    
    let d: u8 = match depth.parse() {
        Ok(d) => d,
        Err(_) => {
            println!("Failed to parse argument to a number.");
            return;
        }
    };
    board::print_move_trees(&board, d);
}
    
fn test_mode_moves(depth: &String, fen: &String, moves: &String) {
    todo!();
}


fn user_mode() { 
    let mut board = board::create_board();
//    board::load_fen(&mut board, "nbqkbnr/1pp1pppp/8/p2pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1");
//    board::load_fen(&mut board, "rnbqkb1r/1ppppppp/5P2/p7/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1");
//    board::load_fen(&mut board, "rnbqkbnr/1pp1pppp/3P4/p7/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1");
//    board::load_fen(&mut board, "rnbqkbnr/2pppppp/p7/Pp6/8/8/1PPPPPPP/RNBQKBNR w KQkq b6 0 1");
//    board::load_fen(&mut board, "rnbqkbnr/ppp2ppp/3p4/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 1");
    println!("What would you like to do m for manual or a number for depth");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    if choice.trim().starts_with("m"){
        loop { 
            let mut start = String::new();
            let mut end = String::new();   
            
            io::stdin().read_line(&mut start).unwrap();
            if start.trim().starts_with("M") {
                board::print_current_moves(&board);           
            } else if start.trim().starts_with("G") {
                board::print_pseudo_moves(&board);
            } else if start.trim().starts_with("A") {
                board::print_attacks(&board);
            } else if start.trim().starts_with("W") {
                board::print_white(&board);
            } else if start.trim().starts_with("B") {
                board::print_black(&board);
            } else if start.trim().starts_with("N") {
                board::print_knights(&board);
            } else if start.trim().starts_with("P") {
                board::print_board(&board);
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
        for _ in 0..num {
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
