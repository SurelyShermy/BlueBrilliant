pub mod board;
pub mod evaluation;
pub mod transposition;

use rand::*;
use transposition::*;

use evaluation::*;
use std::env;
use std::io;
use std::mem;

use crate::board::generate_legal_moves;

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut board = board::create_board();
    let mut evaluator = Evaluation::new();
    if args.len() == 2 {
        board::load_fen(&mut board, args[1].as_str());
    }
    let depth = 15;
    env::set_var("RUST_BACKTRACE", "1");
    while true {
        while board.is_white_move() {
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
            assert!(from_index >= 0, "From index out of bounds, got {}", from_index);
            assert!(from_index < 64, "From index out of bounds, got {}", from_index);
            assert!(to_index >= 0, "To index out of bounds, got {}", to_index);
            assert!(to_index < 64, "To index out of bounds, got {}", to_index);
            let moves = generate_legal_moves(&board);
            let mut valid = false;
            for i in (0..moves.len()).step_by(2){
                if moves[i] == from_index && moves[i+1] == to_index {
                    valid = true;
                    break;
                }
                if i == moves.len() - 2 {
                    println!("Invalid Move, please enter a valid move");
                    continue;
                }
            }
            if !valid {
                println!("Invalid Move, please enter a valid move");
                continue;
            }
            println!("Valid Move, making move ({} -> {})", from_index, to_index);
            board::make_move(&mut board, from_index, to_index);
            board::print_board(&board);
        }      
        while !board.is_white_move() {
            let mut best_move = (0,0);
            let mut eval = 0;
            let mut nodes_counted = 0;
            (eval, best_move, nodes_counted) = evaluation::Evaluation::iterative_deepening_ab_pruning(&mut evaluator, &mut board, i32::MIN, i32::MAX, (0,0), depth, false);
            println!("Eval: {}", eval);
            println!("Best Move: {:?}", best_move);
            println!("Nodes Counted: {}", nodes_counted);
            println!("Exact: {}, Lower: {}, Upper: {}", evaluator.exact_match, evaluator.lower_match, evaluator.upper_match);
            println!("Raw: {}", evaluator.raw_match);
            board::make_move(&mut board, best_move.0, best_move.1);
            board::print_board(&board);
        }
        // if(board::is_checkmate(&mut board)){
        //     println!("Checkmate");
        //     break;
        // }
    }
//     let args: Vec<String> = env::args().collect();
//     if args.len() == 4 {
//         test_mode_moves(&args[1], &args[2], &args[3]); 
//     } else if args.len() == 3 {
//         test_mode(&args[1], &args[2]);
//     } else if args.len() == 2 {
//   //      evaluation_test(&args[1]);
//     } else {
//         user_mode();
//     }

  
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


fn test() { 
    let mut board = board::create_board();
    //ep test
    // board::load_fen(&mut board, "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    //perft 4
    // board::load_fen(&mut board, "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1");
    //perft 2
    board::load_fen(&mut board, "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
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
            } else if start.trim().starts_with("E") {
                board::print_en(&board);
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
