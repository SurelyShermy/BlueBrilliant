pub mod board;
pub mod evaluation;
pub mod transposition;
use std::collections::HashMap;
use std::vec;
use lazy_static::lazy_static;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use board::get_end_index;
use rand::*;
use transposition::*;
use std::collections::VecDeque;
use evaluation::*;

use crate::board::Board;
use axum::{
    extract::ws::{WebSocketUpgrade, WebSocket},
    extract::Path,
    routing::get,
    routing::post,
    http::StatusCode,
    response::{IntoResponse, Response},
    Router,
    Json,
};
use futures::{sink::SinkExt, stream::StreamExt};

use tokio::sync::broadcast;
#[derive(Serialize, Deserialize)]
#[serde(tag = "message_type", content = "data")]
enum WebSocketMessage {
    Initialize(Initialize),
    GameMove(GameMove),
    moves_request(moves_request),
    broadcast(Broadcast_GameState),
}

#[derive(Serialize, Deserialize)]
struct matchmaking_response{
    game_state: Option<GameState>,
    game_id: Option<String>,
    match_found: bool,
}

#[derive(Serialize, Deserialize)]
struct moves_request{
    from_index: u8,
    game_id: String,
}

#[derive(Serialize, Deserialize)]
struct valid_moves{
    message_type: String,
    moves: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct board_enc{
    board: vec::Vec<u8>,

}

#[derive(Clone, Serialize, Deserialize)]
struct GameState{
    message_type: String,
    board: Board,
    game_id: String,
    player1_id: String,
    player2_id: String,
    player1_color: bool,
    player2_color: bool,
    turn: bool,
    board_array: vec::Vec<u8>,
    engine: bool
}
#[derive(Clone, Serialize, Deserialize)]
struct Broadcast_GameState{
    gameId: String,
}
#[derive(Clone, Serialize, Deserialize)]

struct Initialize{
    game_id: String,
}
#[derive(Clone, Serialize, Deserialize)]

struct GameMove {
    game_id: String,
    fromIndex: u8,
    toIndex: u8,
}
#[derive(Deserialize)]
struct MessageType {
    message_type: String,
}
struct PlayerId {
    player_id: String,
}

fn create_pvp_game(player1_id: String, player2_id: String) -> Json<GameState> {
    
    let id = generate_unique_id();
    let (player1_color, player2_color) = assign_player_colors();
    let new_board = board::create_board();
    let gameState = GameState{
        message_type: "GameState".to_string(),
        board: new_board.clone(),
        game_id: id.clone(),
        player1_id: player1_id.clone(),
        player2_id: player2_id.clone(),
        player1_color: player1_color,
        player2_color: player2_color,
        turn: new_board.is_white_move(), //should always be true but just in case
        board_array: board::board_enc(&new_board),
        engine: false,

    };
    insert_gameState(id.clone(), gameState.clone());
    Json(gameState)

}
fn assign_player_colors() -> (bool, bool) {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.5) {  // There's a 50% chance for this to be true
        (true, false)  // Player 1 is black, Player 2 is white
    } else {
        (false, true)  // Player 2 is black, Player 1 is white
    }
}

async fn create_engine_game(Path(PlayerId { player_id }): Path<PlayerId>) -> axum::Json<GameState> {
    let id = generate_unique_id();
    let mut new_board = board::create_board();
    let gameState = GameState{
        message_type: "GameState".to_string(),
        board: new_board.clone(),
        game_id: id.clone(),
        player1_id: player_id.clone(),
        player2_id: "engine".to_string(),
        player1_color: false,
        player2_color: false,
        turn: new_board.is_white_move(),
        board_array: board::board_enc(&new_board.clone()),
        engine: true
    };
    insert_gameState(id.clone(), gameState.clone());
    Json(gameState)
}

//endpoint for player color selection modal
#[post("/engine_game/<game_id>/<player_color>")]
fn set_player_color(game_id: String, player_color: String) -> Json<GameState> {
    let color = player_color == "white";
    GAMESTATES.lock().unwrap().get_mut(&game_id).unwrap().player1_color = color;
    GAMESTATES.lock().unwrap().get_mut(&game_id).unwrap().player2_color = !color;
    Json(GAMESTATES.lock().unwrap().get(&game_id).unwrap().clone())

}

//endpoint for move piece
#[post("/game/<id>/move/<from_index>/<to_index>")]
fn user_make_move(id: String, from_index: u8, to_index: u8)->String{
    let current_board = get_board(id.clone());
    let mut board = current_board.unwrap();
    board::make_move(&mut board, from_index, to_index);
    let ret = update_board(id, board.clone());
    serde_json::to_string(&ret).unwrap()
}
//endpoint for engine making a move
#[post("/game/<id>/engine_move")]
fn engine_move(id: String)->Json<board_enc>{
    let current_board = get_board(id.clone());
    let mut board = current_board.unwrap();
    board::print_board(&board);
    let mut evaluator = Evaluation::new();
    let depth = 6;
    let mut best_move = (0,0);
            let mut eval = 0;
            let mut nodes_counted = 0;
            (eval, best_move, nodes_counted) = evaluation::Evaluation::iterative_deepening_ab_pruning(&mut evaluator, &mut board, i32::MIN, i32::MAX, (0,0), depth, false);
    board::make_move(&mut board, best_move.0, best_move.1);
    board::print_board(&board);
    
    update_board(id, board.clone());
    Json(board_enc{
        board: board::board_enc(&board),
    })
}
struct AppState {
    GAMESTATES: Mutex<HashMap<String, GameState>>,
    MATCHMAKING_QUEUE: Mutex<VecDeque<String>>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        AppState {
            GAMESTATES: Mutex::new(self.GAMESTATES.lock().unwrap().clone()),
            MATCHMAKING_QUEUE: Mutex::new(self.MATCHMAKING_QUEUE.lock().unwrap().clone()),
        }
    }
}

fn insert_gameState(key: String, gameState: GameState) {
    let mut map = GAMESTATES.lock().unwrap();
    map.insert(key.to_owned(), gameState);
}

fn update_board(key: String, board: Board)-> GameState{
    let mut map = GAMESTATES.lock().unwrap();
    let game_state = map.get_mut(&key).expect("Error: Game state not found for the given key");
    game_state.board = board.clone();
    game_state.turn = board.is_white_move();
    game_state.board_array = board::board_enc(&board);
    game_state.clone()
}
fn get_board(key: String) -> Option<Board> {
    let map = GAMESTATES.lock().unwrap();
    let game_state = map.get(&key);
    if game_state.is_none() {
        return None;
    }
    Some(game_state.unwrap().board.clone())
}

#[get("/game/<id>/valid_moves/<start>")]
fn send_valid_moves(id: String, start: u8)->String{
    let mut calculated_moves;
    println!("id: {}", id);
    
    unsafe{
        let mut board_guard = GAMESTATES.lock().unwrap();
        let mut gameState = board_guard.get_mut(&id).unwrap();
        calculated_moves = get_end_index(&gameState.board.clone(), start);
        drop(board_guard);
    }
    let valid_moves = valid_moves{
        message_type: "valid_moves".to_string(),
        moves: calculated_moves,
    };
    serde_json::to_string(&valid_moves).unwrap()
}


async fn matchmaking(Path(PlayerId { player_id }): Path<PlayerId>, Json(payload): Json<matchmaking_response>) -> impl IntoResponse {
    
    let mut games_lock = GAMESTATES.lock().unwrap();
    if let Some((game_id, game_state)) = games_lock.iter().find(|(_, gs)| gs.player1_id == player_id || gs.player2_id == player_id) {
        return Json(matchmaking_response {
            game_id: Some(game_id.clone()),
            match_found: true,
            game_state: Some(game_state.clone()),
        });
    }
    drop(games_lock);
    let mut queue = MATCHMAKING_QUEUE.lock().unwrap();
    if !queue.contains(&player_id) {
        queue.push_back(player_id.clone());
    }

    if queue.len() >= 2 {
        let player1 = queue.pop_front().unwrap();
        let player2 = queue.pop_front().unwrap();
        let game_state_json = create_pvp_game(player1, player2);

        let game_state: GameState = game_state_json.0;

        Json(matchmaking_response {
            game_id: Some(game_state.game_id.clone()),
            match_found: true,
            game_state: Some(game_state),
        })
    } else {
        Json(matchmaking_response {
            game_id: None,
            match_found: false,
            game_state: None,
        })
    }
}

fn generate_unique_id()-> String{
    static mut COUNTER: u64 = 0;
    unsafe{
        COUNTER += 1;
        COUNTER.to_string()
    }
}
#[tokio::main]
async fn main() {


    let GAMESTATES = Mutex::new(HashMap::new());
    let MATCHMAKING_QUEUE = Mutex::new(VecDeque::new());
    let AppState = AppState {
        GAMESTATES,
        MATCHMAKING_QUEUE,
    };
    let app = Router::new()
        .route("/engine_game/:player_id", post(create_engine_game))
        .route("/matchmaking/:player_id", post(matchmaking))
        .with_state(AppState);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}