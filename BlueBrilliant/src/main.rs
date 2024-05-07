extern crate rocket;
pub mod board;
pub mod evaluation;
pub mod transposition;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::vec;
use uuid::{uuid, Uuid};

use std::time::{Instant, Duration};
use lazy_static::lazy_static;
use rocket::http::Method; // Import Method from rocket::http
use rocket_cors::{AllowedOrigins, CorsOptions}; // Import necessary types from rocket_cors
use rand::Rng;
use board::get_end_index;
use rocket::futures::{stream::SplitSink, SinkExt, StreamExt};

use ws::stream::DuplexStream;
use ws::Message;
use std::sync::{Arc};
use tokio::sync::{Mutex, MutexGuard};

use std::collections::VecDeque;

use evaluation::*;
use rocket::{post, get, routes};
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use crate::board::{game_over_check, Board};

#[derive(Serialize, Deserialize)]
#[serde(tag = "message_type", content = "data")]
enum WebSocketMessage {
    Initialize(Initialize),
    GameMove(GameMove),
    moves_request(moves_request),
    broadcast(Broadcast_GameState),
    game_state(GameState),
    EngineMoveRequest(EngineMoveRequest),
    gameOver_request(gameOver_request),
    gameOver_response(gameOver_response),
    resign_request(resign_request),
    time_update(time_update),
    rematch_request(rematch_request),
    rematch_confirmed(rematch_confirmed),
}
#[derive(Serialize, Deserialize)]
struct resign_request{
    game_id: String,
    player: String,
}
#[derive(Clone, Serialize, Deserialize)]
struct rematch_request{
    message_type: String,
}
#[derive(Serialize, Deserialize)]
struct rematch_confirmed{
    game_id: String,
}
#[derive(Serialize, Deserialize)]
struct matchmaking_response{
    game_state: Option<GameState>,
    game_id: Option<String>,
    match_found: bool,
}
#[derive(Serialize, Deserialize)]
struct gameOver_request{
    game_id: String,
}
#[derive(Serialize, Deserialize)]
struct gameOver_response{
    message_type: String,
    result: String,
}

#[derive(Serialize, Deserialize)]
struct moves_request{
    fromIndex: u8,
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
    engine: bool,
    game_over: bool,
    player1_time: u32,
    player2_time: u32,
}
#[derive(Clone, Serialize, Deserialize)]
struct time_update{
    gameId: String,
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
#[derive(Clone, Serialize, Deserialize)]
struct EngineMoveRequest{
    game_id: String,
}
#[derive(Deserialize)]
struct MessageType {
    message_type: String,
}

lazy_static! {
    static ref GAMESTATES: Arc<Mutex<HashMap<String, GameState>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref ENGINEGAMES: Arc<Mutex<HashMap<String, GameState>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref MATCHMAKING_QUEUE: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());
    static ref GAMECHANNELS: Arc<Mutex<HashMap<String, Vec<futures::stream::SplitSink<DuplexStream, ws::Message>>>>> = Arc::new(Mutex::new(HashMap::new()));
}

#[get ("/ws/<game_id>")]
async fn game_ws(game_id: String, ws: ws::WebSocket) -> ws::Channel<'static> {
    use rocket::futures::{SinkExt, StreamExt};
    let owned_game_channel = GAMECHANNELS.clone();
    let mut games ;
    if GAMESTATES.lock().await.contains_key(&game_id){
        games = GAMESTATES.clone();
    }
    else if ENGINEGAMES.lock().await.contains_key(&game_id){
        games = ENGINEGAMES.clone();
    }
    else{
        panic!("Game not found in game states");
    }
    let mut evaluator = Evaluation::new();
    ws.channel(move |mut duplex| Box::pin(async move {
        let (mut sink, mut stream) = duplex.split();
        let sink_id = {
            let mut mutex = owned_game_channel.lock().await;
            let sinks_mutex= mutex.get_mut(&game_id);
            match sinks_mutex{
                Some(sinks) => {
                    let sink_id = sinks.len();
                    sinks.push(sink);
                    sink_id
                },
                None => {panic!("Game not found in sink mutex!")}
            }             
        };
        let mut last_tick = Instant::now();
        let tick_duration = Duration::from_secs(1);
        while let Some(message) = stream.next().await {
            match message{
                Ok(ws::Message::Text(text)) =>{
                    match serde_json::from_str::<WebSocketMessage>(&text){
                        Ok(WebSocketMessage::Initialize(initialize)) => {
                            let game_state = games.lock().await.get(&initialize.game_id).unwrap().clone();
                            let game_state_json = serde_json::to_string(&game_state).expect("Failed to serialize game state");
                            let mut mutex = owned_game_channel.lock().await;
                            let sinks_mutex = mutex.get_mut(&game_id);
                            match sinks_mutex{
                                Some(sinks) => {
                                    let sink = sinks.get_mut(sink_id);
                                    match sink{
                                        Some(sink) => {
                                            sink.send(ws::Message::Text(game_state_json)).await.unwrap();
                                        },
                                        None => {panic!("Sink not found!")}
                                    }
                                },
                                None => {}
                            }
                        },
                        Ok(WebSocketMessage::GameMove(game_move)) => {
                            // Handle game move
                            let mut map = games.lock().await;
                            let game_state = user_make_move(&mut map, game_move.game_id.clone(), game_move.fromIndex, game_move.toIndex).await;
                            println!("Broadcasting: {}", game_state);
                            let mut mutex = owned_game_channel.lock().await;
                            
                            broadcast_game_update(&mut mutex, &game_move.game_id, &game_state).await;
                        },
                        Ok(WebSocketMessage::moves_request(moves_request)) => {
                            // Handle moves request
                            let mut map = games.lock().await;
                            let result = send_valid_moves(&mut map, moves_request.game_id, moves_request.fromIndex).await;
                            // let result_to_send = serde_json::to_string(&result).expect("Failed to serialize moves array");
                            let mut mutex = owned_game_channel.lock().await;
                            let sinks_mutex = mutex.get_mut(&game_id);
                            match sinks_mutex{
                                Some(sinks) => {
                                    let sink = sinks.get_mut(sink_id);
                                    match sink{
                                        Some(sink) => {
                                            sink.send(ws::Message::Text(result)).await.unwrap();
                                        }
                                        None => {}
                                    }
                                },
                                None => {}
                            }
                        },
                        Ok(WebSocketMessage::broadcast(broadcast)) => {
                            // Handle broadcast
                            let game_state = games.lock().await.get(&broadcast.gameId).unwrap().clone();
                            let game_state_json = serde_json::to_string(&game_state).expect("Failed to serialize game state");
                            let mut mutex = owned_game_channel.lock().await;
                            let sinks_mutex = mutex.get_mut(&game_id);
                            match sinks_mutex{
                                Some(sinks) => {
                                    for sink in sinks.iter_mut(){
                                        sink.send(ws::Message::Text(game_state_json.clone())).await.unwrap();
                                    }
                                },
                                None => {}
                            }
                        },
                        Ok(WebSocketMessage::EngineMoveRequest(engine_move_request)) => {
                            // Handle engine move request
                            println!("Processing Move Request");
                            let mut map = games.lock().await;
                            let game_state = engine_move(&mut map, engine_move_request.game_id.clone(), &mut evaluator).await;
                            let mut mutex = owned_game_channel.lock().await;
                            let sinks_mutex = mutex.get_mut(&game_id);
                            match sinks_mutex{
                                Some(sinks) => {
                                    for sink in sinks.iter_mut(){
                                        sink.send(ws::Message::Text(game_state.clone())).await.unwrap();
                                    }
                                },
                                None => {}
                            }
                        },
                        //Game ending handling, the client will close the socket to remove the game from the hashmap
                        Ok(WebSocketMessage::gameOver_request(gameOver_request)) => {
                            let mut map = games.lock().await;
                            let game_state = map.get_mut(&gameOver_request.game_id);
                            match game_state{
                                Some(game_state) => {
                                    let mut game_result: String = game_over_check(&mut game_state.board.clone());
                                    if game_result == "False"{
                                        if game_state.player1_time == 0 || game_state.player2_time == 0{
                                            if game_state.player1_time == 0{
                                                game_result = game_state.player2_id.clone() + " wins on time";
                                            }else{
                                                game_result = game_state.player1_id.clone() + " wins on time";
                                            }
                                            game_state.game_over = true;
                                        }
                                    }else if game_result == "Black wins"{
                                        if game_state.player1_color{
                                            game_result = format!("{} wins", game_state.player2_id.clone());
                                        }else{
                                            game_result = format!("{} wins", game_state.player1_id.clone());
                                        }
                                    }else if game_result == "White wins"{
                                        if game_state.player1_color{
                                            game_result = format!("{} wins", game_state.player1_id.clone());
                                        }else{
                                            game_result = format!("{} wins", game_state.player2_id.clone());
                                        }
                                    }
                                    let game_over_response = gameOver_response{
                                        message_type: "gameOver_response".to_string(),
                                        result: game_result.clone(),
                                    };

                                    let game_over_response_json = serde_json::to_string(&game_over_response).expect("Failed to serialize game over response");
                                    let mut mutex = owned_game_channel.lock().await;
                                    let mut sinks_mutex = mutex.get_mut(&game_id);
                                    match sinks_mutex{
                                        Some(sinks) =>{
                                            for sink in sinks.iter_mut(){
                                                sink.send(ws::Message::Text(game_over_response_json.clone())).await.unwrap();
                                            }
                                        },
                                        None => {}
                                    }
                                },
                                None => {}
                            }
                        },
                        Ok(WebSocketMessage::resign_request(resign_request)) => {
                            let mut map = games.lock().await;
                            let game_state_option = map.get_mut(&resign_request.game_id);
                            let mut loser= "".to_string();
                            let mut winner = "".to_string();
                            match game_state_option{
                                Some(game_state) => {
                                    game_state.game_over = true;
                                    loser = if game_state.player1_id == resign_request.player{
                                        game_state.player1_id.clone()
                                    }else{
                                        game_state.player2_id.clone()
                                    };
                                    winner = if game_state.player1_id == resign_request.player{
                                        game_state.player2_id.clone()
                                    }else{
                                        game_state.player1_id.clone()
                                    };

                                },
                                None => {}
                            }
                            let game_result = format!("{} {} resigned", winner, loser);
                            let game_over_response = gameOver_response{
                                message_type: "gameOver_response".to_string(),
                                result: game_result.clone(),
                            };
                            let game_over_response_json = serde_json::to_string(&game_over_response).expect("Failed to serialize game over response");
                            let mut mutex = owned_game_channel.lock().await;
                            let sinks_mutex = mutex.get_mut(&game_id);
                            match sinks_mutex{
                                Some(sinks) => {
                                    for sink in sinks.iter_mut(){
                                        sink.send(ws::Message::Text(game_over_response_json.clone())).await.unwrap();
                                    }
                                },
                                None => {}
                            }
                        },
                        Err(e) => {
                            eprintln!("Error parsing message: {:?}", e);
                        },
                        Ok(WebSocketMessage::rematch_request(rematch_request)) =>{
                            let mut mutex = owned_game_channel.lock().await;
                            let sinks_mutex = mutex.get_mut(&game_id);
                            let rematch_response = rematch_request{
                                message_type: "rematch_request".to_string(),
                            };
                            let rematch_response_json = serde_json::to_string(&rematch_response).expect("Failed to serialize rematch request");
                            match sinks_mutex{
                                Some(sinks) => {
                                    for sink in sinks.iter_mut(){
                                        sink.send(ws::Message::Text(rematch_response_json.clone())).await.unwrap();
                                    }
                                },
                                None => {}
                            }

                        }
                        Ok(WebSocketMessage::rematch_confirmed(rematch_confirmed)) => {
                            let mut map = games.lock().await;
                            let game_state_option = map.get_mut(&rematch_confirmed.game_id);
                            match game_state_option{
                                Some(game_state) => {
                                    game_state.board = board::create_board();
                                    game_state.turn = game_state.board.is_white_move();
                                    game_state.board_array = board::board_enc(&game_state.board);
                                    game_state.game_over = false;
                                    game_state.player1_time = 600;
                                    game_state.player2_time = 600;
                                    let game_state_json = serde_json::to_string(&game_state).expect("Failed to serialize game state");
                                    let mut mutex = owned_game_channel.lock().await;
                                    let sinks_mutex = mutex.get_mut(&game_id);
                                    match sinks_mutex{
                                        Some(sinks) => {
                                            for sink in sinks.iter_mut(){
                                                sink.send(ws::Message::Text(game_state_json.clone())).await.unwrap();
                                            }
                                        },
                                        None => {}
                                    }
                                },
                                None => {}
                            }
                        },
                        Ok(WebSocketMessage::time_update(time_update)) => {
                            let mut game_states = games.lock().await;
                            let mut result = "".to_string();
                            if let Some(game_state) = game_states.get_mut(&game_id) {
                                if game_state.turn {
                                    game_state.player1_time -= 1;
                                    if game_state.player1_time == 0 {
                                        result = format!("{} wins on time", game_state.player2_id.clone());
                                        game_state.game_over = true;
                                        let game_over_response = gameOver_response{
                                            message_type: "gameOver_response".to_string(),
                                            result: result.clone(),
                                        };
                                        let game_over_response_json = serde_json::to_string(&game_over_response).expect("Failed to serialize game over response");
                                        let mut mutex = owned_game_channel.lock().await;
                                        let sinks_mutex = mutex.get_mut(&game_id);
                                        match sinks_mutex{
                                            Some(sinks) => {
                                                for sink in sinks.iter_mut(){
                                                    sink.send(ws::Message::Text(game_over_response_json.clone())).await.unwrap();
                                                }
                                            },
                                            None => {}
                                        }
                                    }
                                } else {
                                    game_state.player2_time -= 1;
                                    if game_state.player2_time == 0 {
                                        game_state.game_over = true;
                                        result = format!("{} wins on time", game_state.player1_id.clone());

                                        let game_over_response = gameOver_response{
                                            message_type: "gameOver_response".to_string(),
                                            result: result.clone(),
                                        };
                                        let game_over_response_json = serde_json::to_string(&game_over_response).expect("Failed to serialize game over response");
                                        let mut mutex = owned_game_channel.lock().await;
                                        let sinks_mutex = mutex.get_mut(&game_id);
                                        match sinks_mutex{
                                            Some(sinks) => {
                                                for sink in sinks.iter_mut(){
                                                    sink.send(ws::Message::Text(game_over_response_json.clone())).await.unwrap();
                                                }
                                            },
                                            None => {}
                                        }
                                    }
                                }
                                // Broadcast time update to both players
                                let time_update = serde_json::to_string(&game_state).expect("Failed to serialize game state");
                                let mut mutex = owned_game_channel.lock().await;
                                if let Some(sinks) = mutex.get_mut(&game_id) {
                                    for sink in sinks.iter_mut() {
                                        sink.send(ws::Message::Text(time_update.clone())).await.unwrap();
                                    }
                                }
                            }
                        },
                        _=>(),
                    }
                },
                Ok(ws::Message::Ping(data)) => (),
                Ok(ws::Message::Close(_)) => {
                    let mut games = GAMECHANNELS.lock().await;
                    match games.get_mut(&game_id){
                        Some(sinks) => {
                            for sink in sinks.iter_mut(){
                                sink.close().await.unwrap();
                            }
                            for sink in sinks.len()..0{
                                sinks.pop();
                            }
                        },
                        None => {}
                    }
                    let mut game_states = GAMESTATES.lock().await;
                    let game_state_option = game_states.get(&game_id);
                    match game_state_option {
                        Some(value) => {
                            if value.game_over{
                                game_states.remove(&game_id);
                            }
                        }
                        None => {}
                    }
                    break;
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                    // Handle the error
                },
                _ => (),

            }
        }
        Ok(())
    }))
}

async fn broadcast_game_update(map: & mut MutexGuard<'_, HashMap<String, Vec<SplitSink<DuplexStream, Message>>>>, game_id: &String, broadcast_json: &String){
    let senders = map.get_mut(game_id);
    match senders{
        Some(senders) => {
            let broadcast_json = ws::Message::Text(broadcast_json.clone());
            for sender in senders{
                sender.send(broadcast_json.clone()).await.unwrap();
            }
        },
        None => {}
    }
}

async fn create_pvp_game(player1_id: String, player2_id: String) -> Json<GameState> {
    
    let id = Uuid::new_v4().to_string();
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
        player1_time: 600,
        player2_time: 600,
        engine: false,
        game_over: false,
    }; 
    GAMECHANNELS.lock().await.insert(id.clone(), Vec::new());
    insert_gameState(&mut GAMESTATES.lock().await, id.clone(), gameState.clone()).await;
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

#[post("/engine_game/<player_id>")]
async fn create_engine_game(player_id: String) -> Json<GameState> {
    let id = Uuid::new_v4().to_string();
    let mut new_board = board::create_board();
    let (player1_color, player2_color) = assign_player_colors();
    // let player1_color = true;
    // let player2_color = false;
    // let fen = "8/8/8/8/8/3k4/7q/3K4 b - - 0 1";
    // board::load_fen(&mut new_board, fen);
    let gameState = GameState{
        message_type: "GameState".to_string(),
        board: new_board.clone(),
        game_id: id.clone(),
        player1_id: player_id.clone(),
        player2_id: "engine".to_string(),
        player1_color: player1_color,
        player2_color: player2_color,
        turn: new_board.is_white_move(),
        board_array: board::board_enc(&new_board.clone()),
        player1_time: 600,
        player2_time: 600,
        engine: true,
        game_over: false,
    };
    GAMECHANNELS.lock().await.insert(id.clone(), Vec::new());
    insert_gameState(&mut ENGINEGAMES.lock().await, id.clone(), gameState.clone()).await;
    Json(gameState)
}

// //endpoint for player color selection modal

// fn set_player_color(map: &mut MutexGuard<'_, HashMap<String, GameState>>, game_id: String, player_color: String) -> Json<GameState> {
//     let color = player_color == "white";
//     map.get_mut(&game_id).unwrap().player1_color = color;
//     map.get_mut(&game_id).unwrap().player2_color = !color;
//     Json(map.get(&game_id).unwrap().clone())

// }


async fn user_make_move(map: &mut MutexGuard<'_, HashMap<String, GameState>>, id: String, from_index: u8, to_index: u8)->String{
    let current_board = get_board(map, id.clone());
    let mut board = current_board.unwrap();
    board::make_move(&mut board, from_index, to_index);
    let ret = update_board(map, id, board.clone()).await;
    serde_json::to_string(&ret).unwrap()
}
//endpoint for engine making a move
async fn engine_move(map: &mut MutexGuard<'_, HashMap<String, GameState>>, id: String, evaluator: &mut evaluation::Evaluation)->String{
    let current_board = get_board(map, id.clone());
    let mut maximizer = false;
    let mut board = current_board.unwrap();
    if board.is_white_move() {
        maximizer = true;
    } else {
        maximizer = false;
    }
    board::print_board(&board);
    let depth = 15;
    let mut best_move = (0,0);
    let mut eval = 0;
    let mut nodes_counted = 0;
    println!("Called iterative deepning");
    (eval, best_move, nodes_counted) = evaluation::Evaluation::iterative_deepening_ab_pruning( evaluator, &mut board, i32::MIN, i32::MAX, (0,0), depth, maximizer);
    board::make_move(&mut board, best_move.0, best_move.1);
    board::print_board(&board);
    println!("{},{}", best_move.0, best_move.1);
    serde_json::to_string(&update_board(map, id, board.clone()).await).unwrap()
}

async fn insert_gameState(map: & mut MutexGuard<'_, HashMap<String, GameState>>, key: String, gameState: GameState) {
    map.insert(key.to_owned(), gameState);
}


async fn update_board(map: & mut MutexGuard<'_, HashMap<String, GameState>>, key: String, board: Board)-> GameState{
    let game_state = map.get_mut(&key).expect("Error: Game state not found for the given key");
    game_state.board = board.clone();
    game_state.turn = board.is_white_move();
    game_state.board_array = board::board_enc(&board);
    game_state.clone()
}
fn get_board(map: &MutexGuard<HashMap<String, GameState>>, key: String) -> Option<Board> {
    let game_state = map.get(&key);
    if game_state.is_none() {
        return None;
    }
    Some(game_state.unwrap().board.clone())
}


async fn send_valid_moves(map:& mut MutexGuard<'_, HashMap<String, GameState>>, id: String, start: u8)->String{
    let mut calculated_moves;
    println!("id: {}", id);
    let mut gameState = map.get_mut(&id).unwrap();
    calculated_moves = get_end_index(&gameState.board.clone(), start);
    // unsafe{
    //     let mut board_guard = GAMESTATES.lock().await;
    //     let mut gameState = board_guard.get_mut(&id).unwrap();
    //     calculated_moves = get_end_index(&gameState.board.clone(), start);
    //     drop(board_guard);
    // }
    let valid_moves = valid_moves{
        message_type: "valid_moves".to_string(),
        moves: calculated_moves,
    };
    serde_json::to_string(&valid_moves).unwrap()
}

#[post("/matchmaking/<player_id>")]
async fn matchmaking(player_id: String) -> Json<matchmaking_response> {
    println!("Matchmaking called for player {}", player_id);
    let mut games_lock = GAMESTATES.lock().await;
    if let Some((game_id, game_state)) = games_lock.iter().find(|(_, gs)| gs.player1_id == player_id || gs.player2_id == player_id) {
        return Json(matchmaking_response {
            game_id: Some(game_id.clone()),
            match_found: true,
            game_state: Some(game_state.clone()),
        });
    }
    drop(games_lock);
    let mut queue = MATCHMAKING_QUEUE.lock().await;
    if !queue.contains(&player_id) {
        queue.push_back(player_id.clone());
    }

    if queue.len() >= 2 {
        println!("Players in queue");
        let player1 = queue.pop_front().unwrap();
        let player2 = queue.pop_front().unwrap();
        let game_state_json = create_pvp_game(player1, player2).await;

        let game_state: GameState = game_state_json.0;

        Json(matchmaking_response {
            game_id: Some(game_state.game_id.clone()),
            match_found: true,
            game_state: Some(game_state),
        })
    } else {
        println!("Not enough players in queue");
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

#[rocket::main]
async fn main() {
    // AllowedOrigins is a list of origins that are allowed to make requests
    // You can also specify particular origins like so:
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:4000", "http://localhost:8080", "https://localhost", "http://159.203.107.176", "http://bluebrilliant.me"]);

    let cors = CorsOptions { // Create a CorsOptions instance
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Options].into_iter().map(From::from).collect(),
        allowed_headers: rocket_cors::AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors().unwrap(); // Convert CorsOptions to Cors fairing

    if let Err(e) = rocket::build()
        .attach(cors)
        .mount("/", routes![game_ws, matchmaking, create_engine_game])
        .launch()
        .await
    {
        eprintln!("Rocket launch failed: {:?}", e);
    }
}
