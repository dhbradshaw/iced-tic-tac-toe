use iced::{executor, Application, Clipboard, Command, Element, Settings, Text, Button};
use iced::Column;
use iced::Row;
use iced::button;

pub fn main() -> iced::Result {
    Game::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
pub enum Messages {
    MoveMade(u8)
}

#[derive(Debug, Clone, Copy)]
enum SpotType {
    Empty,
    X,
    O
}

impl SpotType {
    fn to_char(&self) -> char {
        match self {
            SpotType::Empty => ' ',
            SpotType::X => 'X',
            SpotType::O => 'O'
        }
    }
}

struct Game {
    moves: Vec<u8>,

}

impl Game {
    fn new() -> Self {
        Game {
            moves: vec![0,1,2,3,4,5,6,7,8],

        }
    }
    pub fn spots(&self) -> [SpotType; 9] {
        let mut spots = [SpotType::Empty; 9];
        let mut x = true;
        for n in &self.moves {
            if x {
                spots[*n as usize] = SpotType::X;
            } else {
                spots[*n as usize] = SpotType::O;
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
                if self.moves.contains(&move_) {
                    self.moves.push(move_);
                }
            }
        };
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let spots = self.spots();
        let mut spot_elements = Vec::new();
        for (i, spot) in spots.iter().enumerate() {
            let spot_type = spot.to_char();
            let spot_element = Text::new(format!("{spot_type} {i}"))
                .size(50)
                .horizontal_alignment(iced::HorizontalAlignment::Center)
                .vertical_alignment(iced::VerticalAlignment::Center);
            spot_elements.push(spot_element);
        }
        spot_elements.reverse();
        let row_0 = Row::new()
        .padding(10)
        .spacing(10)
        .push(spot_elements.pop().unwrap())
        .push(spot_elements.pop().unwrap())
        .push(spot_elements.pop().unwrap());
        let row_1 = Row::new()
        .padding(10)
        .spacing(10)
        .push(spot_elements.pop().unwrap())
        .push(spot_elements.pop().unwrap())
        .push(spot_elements.pop().unwrap());
        let row_2 = Row::new()
        .padding(10)
        .spacing(10)
        .push(spot_elements.pop().unwrap())
        .push(spot_elements.pop().unwrap())
        .push(spot_elements.pop().unwrap());
        let column = Column::new()
        .padding(10)
        .spacing(10)
        .push(row_0)
        .push(row_1)
        .push(row_2);
        column.into()
    }

}
