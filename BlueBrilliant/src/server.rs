extern crate rocket;
pub mod board;
pub mod evaluation;
pub mod transposition;
use std::collections::HashMap;
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
struct Message{
    message: String,
}

#[post("/board")]
fn create_board() -> Json<String> {
    // Generate a new ID (you can use any method you prefer to generate unique IDs)
    let id = generate_unique_id();

    // Create a new board with some initial content
    let mut new_board = board::create_board();

    // Store the new board in the HashMap
    insert_board(id.clone(), new_board);

    // Return the ID as JSON response
    Json(id)
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
fn send_valid_moves(id: String, start: u8)->Json<Move_Request>{
    let mut moves;
    unsafe{
        let mut board_guard = BOARDS.lock().unwrap();
        let mut board = board_guard.get_mut(&id).unwrap();
        moves = get_end_index(board, start);
    }
    
    
    Json(Move_Request{
        from_index: 0,
        to_index: 0,
        game_id: 0,
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
        .mount("/", routes![index, gamer]) // Mount both routes at the root
        .launch()
        .await
        .unwrap();
}

