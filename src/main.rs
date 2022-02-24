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
use iced_tic_tac_toe::shape_2d;

pub fn main() -> iced::Result {
    Game::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    MoveMade(u8),
    Reset,
    Undo,
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

    // local state
    cell_button_states: [button::State; 9],
    reset_button_state: button::State,
    undo_button_state: button::State,
}

impl Game {
    fn new() -> Self {
        Game {
            moves: vec![],
            cell_button_states: [button::State::new(); 9],
            reset_button_state: button::State::new(),
            undo_button_state: button::State::new(),
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
                let repeat = self.moves.contains(&move_);
                let last = Some(&move_) == self.moves.last();
                match (repeat, last) {
                    (true, true) => {
                        self.moves.pop();
                    }
                    (true, false) => {
                        if !self.winning_lines().is_empty() {
                            self.moves.clear();
                        }
                    }
                    (false, _) => {
                        self.moves.push(move_);
                    }
                }
            }
            Message::Undo => {
                self.moves.pop();
            }
            Message::Reset => {
                self.moves.clear();
            }
        };
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let winning_squares = self.winning_squares();

        // Create the current player message.
        let message = if winning_squares.is_empty() {
            format!("{} next", self.current_player().to_char())
        } else {
            format!(
                "{} wins!",
                if self.moves.len() % 2 == 0 { "O" } else { "X" }
            )
        };
        let current_player = Text::new(message).size(70);

        // Get 2D array of buttons
        let spots = self.spots();
        let mut spot_elements = Vec::new();
        for (i, state) in self.cell_button_states.iter_mut().enumerate() {
            let spot_type = spots[i];
            let winning = winning_squares.contains(&(i as u8));
            spot_elements.push(spot_element(state, spot_type, i as u8, winning));
        }
        let spot_elements = shape_2d(spot_elements, 3);

        // Convert 2D array of buttons to a column of rows.
        let rows = spot_elements.into_iter().map(|e| row(e));
        let mut column = Column::new()
            .padding(0)
            .spacing(0)
            .align_items(Align::Center);
        for row in rows.into_iter() {
            column = column.push(row);
        }

        // Create the reset button.
        let reset_button = Button::new(&mut self.reset_button_state, Text::new("Reset").size(70))
            .on_press(Message::Reset);

        // Create the undo button.
        let undo_button = Button::new(&mut self.undo_button_state, Text::new("Undo").size(70))
            .on_press(Message::Undo);

        // Add the current player message to the bottom of the column
        column = column.push(current_player);
        column = column.push(undo_button);
        column = column.push(reset_button);

        column.into()
    }
}

fn row<'a>(buttons: Vec<Button<'a, Message>>) -> Row<'a, Message> {
    // padding works above and below a row, so it doubles up between rows.  It controls vertical spacing.
    let row_padding = 5;
    // spacing works between row elements, so it does not double up.  It controls horizontal spacing.
    let row_spacing = 2 * row_padding;

    let mut row = Row::new().padding(row_padding).spacing(row_spacing);
    for button in buttons.into_iter() {
        row = row.push(button);
    }
    row.into()
}

fn spot_element(
    state: &mut button::State,
    spot_type: SpotType,
    index: u8,
    winner: bool,
) -> Button<Message> {
    // Style depends on whether the spot is a winning spot or not.
    let size;
    let color;
    if winner {
        size = 90;
        color = iced::Color::from_rgb(0.5, 0.5, 0.5);
    } else {
        size = 80;
        color = iced::Color::from_rgb(0.8, 0.8, 0.8);
    };

    // Pick and style text
    let text = Text::new(spot_type.to_char().to_string())
        .height(Length::Fill)
        .width(Length::Fill)
        .horizontal_alignment(iced::HorizontalAlignment::Center)
        .vertical_alignment(iced::VerticalAlignment::Center)
        .size(size);

    // Style the button
    Button::new(state, text)
        .style(ButtonColor { color })
        .on_press(Message::MoveMade(index))
        .padding(10)
        .height(Length::Units(100))
        .width(Length::Units(100))
}
