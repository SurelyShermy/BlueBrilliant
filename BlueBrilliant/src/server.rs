extern crate rocket;
pub mod board;
pub mod evaluation;
pub mod transposition;

use board::get_end_index;
use rand::*;
use transposition::*;

use evaluation::*;
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;



#[derive(Serialize, Deserialize)]
struct Move_Request{
    from_index: u8,
    to_index: u8,
    game_id: u8,
}

#[post("/board")]
fn create_board() -> Json<String> {
    // Generate a new ID (you can use any method you prefer to generate unique IDs)
    let id = generate_unique_id();

    // Create a new board with some initial content
    let new_board = Board { content: format!("Initial content for board {}", &id) };

    // Store the new board in the HashMap
    BOARDS.insert(id.clone(), new_board);

    // Return the ID as JSON response
    Json(id)
}

lazy_static::lazy_static!{
    static ref BOARDS: Hashmap<String, Board> = {
        let mut map = HashMap::new();
    }
}


#[get("/game/<id>/valid_moves/<start>")]
fn send_valid_moves(id: String, start: u8)->Json<Move_Request>{
    let board = BOARDS.get(&id).unwrap();
    let moves = get_end_index(board, start);
    
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

fn generate_unique_id()-> String{
    static mut COUNTER: u64 = 0;

    unsafe{
        Counter += 1;
        Counter.to_string()
    }
}
#[rocket::main]
async fn main(){
    rocket::build().mount("/", routes![index]).launch().await.unwrap();
}

