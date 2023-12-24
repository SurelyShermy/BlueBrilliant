pub mod game {
    pub mod initialization;
}
pub mod gui; // This is your new GUI module

use game::initialization::*;
use iced::Sandbox;
fn main() -> iced::Result {
    gui::Chessboard::run(iced::Settings::default())
}
