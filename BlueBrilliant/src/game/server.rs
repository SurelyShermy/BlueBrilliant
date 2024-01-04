use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MoveRequest {
    index: usize,
}

#[derive(Serialize, Deserialize)]
struct MoveResponse {
    valid_moves: Vec<usize>,
}
struct MoveInfo {
    from: usize,
    to: usize,
}
async fn calculate_moves(info: web::Json<MoveRequest>) -> impl Responder {
    let valid_moves = moves::calculate_valid_moves(info.index);
    HttpResponse::Ok().json(MoveResponse { valid_moves })
}
async fn make_move(info: web::Json<MoveInfo>) -> impl Responder {
    board::make_move(info.from, info.to);
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/moves", web::post().to(calculate_moves)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
