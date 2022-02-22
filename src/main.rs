use iced::button;
use iced::Column;
use iced::Length;
use iced::Row;
use iced::{executor, Application, Button, Clipboard, Command, Element, Settings, Text};

pub fn main() -> iced::Result {
    Game::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    MoveMade(u8),
}

#[derive(Debug, Clone, Copy)]
enum SpotType {
    Empty,
    X,
    O,
}

impl SpotType {
    fn to_char(&self) -> char {
        match self {
            SpotType::Empty => ' ',
            SpotType::X => 'X',
            SpotType::O => 'O',
        }
    }
}

struct Game {
    moves: Vec<u8>,
    button_states: [button::State; 9],
}

impl Game {
    fn new() -> Self {
        Game {
            moves: vec![],
            button_states: [button::State::new(); 9],
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

    fn current_player(&self) -> SpotType {
        if self.moves.len() % 2 == 0 {
            SpotType::X
        } else {
            SpotType::O
        }
    }
}

impl Application for Game {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Game, Command<Self::Message>) {
        (Game::new(), Command::none())
    }

    fn title(&self) -> String {
        String::from("TicTacToe in Iced")
    }

    fn update(
        &mut self,
        _message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match _message {
            Message::MoveMade(move_) => {
                if !self.moves.contains(&move_) {
                    self.moves.push(move_);
                }
            }
        };
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let current_player = Text::new(self.current_player().to_char());

        let spots = self.spots();
        let mut spot_elements = Vec::new();
        for (i, state) in self.button_states.iter_mut().enumerate() {
            let spot_type = spots[i].to_char();
            let text = Text::new(spot_type)
                .height(Length::Fill)
                .width(Length::Fill)
                .horizontal_alignment(iced::HorizontalAlignment::Center)
                .vertical_alignment(iced::VerticalAlignment::Center)
                .size(80);
            let spot_element = Button::new(state, text)
                .on_press(Message::MoveMade(i as u8))
                .height(Length::Units(100))
                .width(Length::Units(100));
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
            .padding(0)
            .spacing(0)
            .push(current_player)
            .push(row_0)
            .push(row_1)
            .push(row_2);

        column.into()
    }
}
