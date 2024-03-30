extern crate rocket;
pub mod board;
pub mod evaluation;
pub mod transposition;
use std::collections::HashMap;
use std::vec;
use lazy_static::lazy_static;
use std::sync::Mutex;

use board::get_end_index;
use rand::*;
use transposition::*;

use evaluation::*;
use rocket::{post, get, routes};
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use crate::board::Board;



#[derive(Serialize, Deserialize)]
struct Move_Request{
    from_index: u8,
    to_index: u8,
    game_id: u8,
}

#[derive(Serialize, Deserialize)]
struct valid_moves{
    moves: Vec<u8>,
}
#[derive(Serialize, Deserialize)]

struct board_enc{
    board: vec::Vec<u8>,

}

#[derive(Serialize, Deserialize)]
struct Message{
    message: String,
}

#[post("/create_game")]
fn create_game() -> Json<String> {
    // Generate a new ID (you can use any method you prefer to generate unique IDs)
    let id = generate_unique_id();

    // Create a new board with some initial content
    let mut new_board = board::create_board();

    // Store the new board in the HashMap
    insert_board(id.clone(), new_board);

    // Return the ID as JSON response
    Json(id)
}

#[post("/game/<id>/move/<from_index>/<to_index>")]
fn user_make_move(id: String, from_index: u8, to_index: u8)->Json<board_enc>{
    let current_board = get_board(id);
    let mut board = current_board.unwrap();
    board::make_move(&mut board, from_index, to_index);
    Json(board_enc{
        board: board::board_enc(&board),
    })
}
#[post("/game/<id>/engine_move")]
fn engine_move(id: String)->Json<board_enc>{
    let current_board = get_board(id);
    let mut board = current_board.unwrap();
    let mut evaluator = Evaluation::new();
    let depth = 6;
    let mut best_move = (0,0);
            let mut eval = 0;
            let mut nodes_counted = 0;
            (eval, best_move, nodes_counted) = evaluation::Evaluation::iterative_deepening_ab_pruning(&mut evaluator, &mut board, i32::MIN, i32::MAX, (0,0), depth, false);
    board::make_move(&mut board, best_move.0, best_move.1);

    Json(board_enc{
        board: board::board_enc(&board),
    })
}
lazy_static! {
    static ref BOARDS: Mutex<HashMap<String, Board>> = Mutex::new(HashMap::new());
}

fn insert_board(key: String, board: Board) {
    let mut map = BOARDS.lock().unwrap();
    map.insert(key.to_owned(), board);
}

fn get_board(key: String) -> Option<Board> {
    let map = BOARDS.lock().unwrap();
    map.get(&key).cloned()
}

#[get("/game/<id>/valid_moves/<start>")]
fn send_valid_moves(id: String, start: u8)->Json<valid_moves>{
    let mut moves;
    unsafe{
        let mut board_guard = BOARDS.lock().unwrap();
        let mut board = board_guard.get_mut(&id).unwrap();
        moves = get_end_index(board, start);
    }
    Json(valid_moves{
        moves: moves,
    })

}

#[get("/")]
fn index() -> Json<Message> {
    Json(Message {
        message: "Hello, world!".to_string()
    })
}

#[get("/game/<id>")]
fn game(id: u32) -> Json<Message> {
    Json(Message {
        message: format!("Hello, game {}", id)
    })
}

#[get("/gamer")]
fn gamer() -> Json<Message> {
    Json(Message {
        message: "Hello, gamer!".to_string()
    })
}

fn generate_unique_id()-> String{
    static mut COUNTER: u64 = 0;

    unsafe{
        COUNTER += 1;
        COUNTER.to_string()
    }
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![index, create_game]) // Mount both routes at the root
        .launch()
        .await
        .unwrap();
}

