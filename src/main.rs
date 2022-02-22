use iced::{executor, Application, Clipboard, Command, Element, Settings, Text};
use iced::Column;
use iced::Row;

pub fn main() -> iced::Result {
    Game::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
pub enum Messages {
    MoveMade(u8)
}

#[derive(Debug, Clone, Copy)]
enum SpotState {
    Empty,
    X,
    O
}

struct Game {
    moves: Vec<u8>,
}

impl Game {
    fn new() -> Self {
        Game {
            moves: Vec::new(),
        }
    }
    fn spots(&self) -> [SpotState; 9] {
        let mut spots = [SpotState::Empty; 9];
        let mut x = true;
        for n in &self.moves {
            if x {
                spots[*n as usize] = SpotState::X;
            } else {
                spots[*n as usize] = SpotState::O;
            }
            x = !x;
        }
        spots
    }
}

impl Application for Game {
    type Executor = executor::Default;
    type Message = Messages;
    type Flags = ();

    fn new(_flags: ()) -> (Game, Command<Self::Message>) {
        (Game::new(), Command::none())
    }

    fn title(&self) -> String {
        String::from("TicTacToe in Iced")
    }

    fn update(&mut self, _message: Self::Message, _clipboard: &mut Clipboard) -> Command<Self::Message> {
        match _message {
            Messages::MoveMade(move_) => {
                self.moves.push(move_);
            }
        };
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let row_0 = Row::new()
        .padding(10)
        .spacing(10)
        .push(Text::new("| |"))
        .push(Text::new("| |"))
        .push(Text::new("| |"));
        let row_1 = Row::new()
        .padding(10)
        .spacing(10)
        .push(Text::new("| |"))
        .push(Text::new("| |"))
        .push(Text::new("| |"));
        let row_2 = Row::new()
        .padding(10)
        .spacing(10)
        .push(Text::new("| |"))
        .push(Text::new("| |"))
        .push(Text::new("| |"));
        let column = Column::new()
        .padding(10)
        .spacing(10)
        .push(row_0)
        .push(row_1)
        .push(row_2);
        column.into()
    }
}