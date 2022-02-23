use iced::button;
use iced::executor;
use iced::Align;
use iced::Application;
use iced::Button;
use iced::Clipboard;
use iced::Column;
use iced::Command;
use iced::Element;
use iced::Length;
use iced::Row;
use iced::Settings;
use iced::Text;

pub fn main() -> iced::Result {
    Game::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    MoveMade(u8),
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

struct ButtonColor {
    color: iced::Color,
}

impl button::StyleSheet for ButtonColor {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(iced::Background::Color(self.color)),
            ..Default::default()
        }
    }
    // other methods in Stylesheet have a default impl
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

    fn winning_lines(&self) -> Vec<[u8; 3]> {
        let possible_wins = [
            // horizontals
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            // verticals
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            // diagonals
            [0, 4, 8],
            [2, 4, 6],
        ];
        let spots = self.spots();

        let mut wins = vec![];
        for win in possible_wins {
            let values = win.iter().map(|n| spots[*n as usize]).collect::<Vec<_>>();
            if values.iter().all(|v| *v == SpotType::X) {
                wins.push(win);
            } else if values.iter().all(|v| *v == SpotType::O) {
                wins.push(win);
            }
        }
        wins
    }

    fn winning_squares(&self) -> Vec<u8> {
        let mut squares = vec![];
        for win in self.winning_lines() {
            for n in win {
                if !squares.contains(&n) {
                    squares.push(n);
                }
            }
        }
        squares
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
                    if self.winning_lines().is_empty() {
                        self.moves.push(move_);
                    } else {
                        self.moves.clear();
                    }
                }
            }
        };
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let winning_squares = self.winning_squares();
        let message = if winning_squares.is_empty() {
            format!("{} next", self.current_player().to_char())
        } else {
            format!(
                "{} wins!",
                if self.moves.len() % 2 == 0 { "O" } else { "X" }
            )
        };
        let current_player = Text::new(message).size(70);

        let spots = self.spots();
        let mut spot_elements = Vec::new();
        for (i, state) in self.button_states.iter_mut().enumerate() {
            let spot_type = spots[i].to_char();
            let size;
            let color;
            if winning_squares.contains(&(i as u8)) {
                size = 90;
                color = iced::Color::from_rgb(0.5, 0.5, 0.5);
            } else {
                size = 80;
                color = iced::Color::from_rgb(0.8, 0.8, 0.8);
            };
            let text = Text::new(spot_type)
                .height(Length::Fill)
                .width(Length::Fill)
                .horizontal_alignment(iced::HorizontalAlignment::Center)
                .vertical_alignment(iced::VerticalAlignment::Center)
                .size(size);
            let spot_element = Button::new(state, text)
                .style(ButtonColor { color })
                .on_press(Message::MoveMade(i as u8))
                .padding(10)
                .height(Length::Units(100))
                .width(Length::Units(100));
            spot_elements.push(spot_element);
        }
        spot_elements.reverse();

        // padding works above and below a row, so it doubles up between rows.  It controls vertical spacing.
        let row_padding = 5;
        // spacing works between row elements, so it does not double up.  It controls horizontal spacing.
        let row_spacing = 2 * row_padding;

        let row_0 = Row::new()
            .padding(row_padding)
            .spacing(row_spacing)
            .push(spot_elements.pop().unwrap())
            .push(spot_elements.pop().unwrap())
            .push(spot_elements.pop().unwrap());

        let row_1 = Row::new()
            .padding(row_padding)
            .spacing(row_spacing)
            .push(spot_elements.pop().unwrap())
            .push(spot_elements.pop().unwrap())
            .push(spot_elements.pop().unwrap());

        let row_2 = Row::new()
            .padding(row_padding)
            .spacing(row_spacing)
            .push(spot_elements.pop().unwrap())
            .push(spot_elements.pop().unwrap())
            .push(spot_elements.pop().unwrap());

        let column = Column::new()
            .padding(0)
            .spacing(0)
            .align_items(Align::Center)
            .push(row_0)
            .push(row_1)
            .push(row_2)
            .push(current_player);

        column.into()
    }
}
