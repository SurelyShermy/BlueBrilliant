pub mod board;
pub mod evaluation;

use std::env;
use std::io;
use std::mem;

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut board = board::create_board();
    if args.len() == 2 {
        board::load_fen(&mut board, args[1].as_str());
    }
    let depth = 8;
    while true {
        while board.is_white_move {
            let mut start = String::new();
            let mut end = String::new();
            println!("Enter Move: from index, to index");
            io::stdin().read_line(&mut start).unwrap();
            io::stdin().read_line(&mut end).unwrap();
            let start = start.trim();
            let end = end.trim();
            let from_index: u8 = match start.parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("Invalid input for start index. Please enter a valid number.");
                    continue;
                }
            };
            let to_index: u8 = match end.parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("Invalid input for start index. Please enter a valid number.");
                    continue; // Assuming this is in a loop, or otherwise handle the error appropriately
                }
            };
            assert!(from_index > 0, "From index out of bounds, got {}", from_index);
            assert!(from_index < 64, "From index out of bounds, got {}", from_index);
            assert!(to_index > 0, "To index out of bounds, got {}", to_index);
            assert!(to_index < 64, "To index out of bounds, got {}", to_index);
            assert!(board::valid_move(&board, from_index, to_index), "Invalid move");
            println!("Valid Move, making move ({} -> {})", from_index, to_index);
            board::make_move(&mut board, from_index, to_index);
            board::print_board(&board);
        }      
        while !board.is_white_move {
            let mut best_move = (0,0);
            let mut eval = 0;
            let mut nodes_counted = 0;
            (eval, best_move, nodes_counted) = evaluation::ab_pruning(&mut board, i32::MIN, i32::MAX, (0,0), depth, false);
            println!("Eval: {}", eval);
            println!("Best Move: {:?}", best_move);
            println!("Nodes Counted: {}", nodes_counted);
            board::make_move(&mut board, best_move.0, best_move.1);
            board::print_board(&board);
        }
        // if(board::is_checkmate(&mut board)){
        //     println!("Checkmate");
        //     break;
        // }
    }
}
