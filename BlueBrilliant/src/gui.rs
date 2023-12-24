// In src/gui.rs
use crate::game::initialization::*;

use iced::{button, Button, Column, Container, Element, Length, Row, Sandbox, Settings, Text};

pub fn main() -> iced::Result {
    Chessboard::run(Settings::default())
}

pub struct Chessboard {
    board_buttons: [[button::State; 8]; 8], // Buttons for each square
}

impl Sandbox for Chessboard {
    type Message = ();

    fn new() -> Self {
        Self {
            board_buttons: Default::default(),
        }
    }

    fn title(&self) -> String {
        String::from("Chessboard")
    }

    fn update(&mut self, _message: Self::Message) {
        // Update logic here
    }

    fn view(&mut self) -> Element<Self::Message> {
        let mut board = Column::new().spacing(2);

        for row in 0..8 {
            let mut board_row = Row::new().spacing(2);
            for col in 0..8 {
                // Obtain a mutable reference to the button state
                let button_state = &mut self.board_buttons[row][col];

                // Create the button with the state
                let mut button = Button::new(button_state, Text::new(" "))
                    .width(Length::Units(50))
                    .height(Length::Units(50));

                // Apply styling based on the position
                button = if (row + col) % 2 == 0 {
                    button.style(ChessboardStyle::Dark)
                } else {
                    button.style(ChessboardStyle::Light)
                };

                // Add the button to the row
                board_row = board_row.push(button);
            }
            // Add the row to the board
            board = board.push(board_row);
        }

        Container::new(board)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
#[derive(PartialEq, Eq)]
struct ChessboardStyle;

impl button::StyleSheet for ChessboardStyle {
    fn active(&self) -> button::Style {
        match self {
            &ChessboardStyle::Dark => button::Style {
                background: Some(iced::Color::from_rgb(0.63, 0.63, 0.63).into()),
                text_color: iced::Color::WHITE,
                ..button::Style::default()
            },
            &ChessboardStyle::Light => button::Style {
                background: Some(iced::Color::WHITE.into()),
                text_color: iced::Color::BLACK,
                ..button::Style::default()
            },
        }
    }
}

impl ChessboardStyle {
    const Dark: Self = Self;
    const Light: Self = Self;
}
