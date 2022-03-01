use iced::button;
use iced::executor;
use iced::Align;
use iced::Application;
use iced::Button;
use iced::Clipboard;
use iced::Column;
use iced::Command;
use iced::Container;
use iced::Element;
use iced::Length;
use iced::Row;
use iced::Settings;
use iced::Svg;
use iced::Text;

const BASE_SIZE: u16 = 6;

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
enum BoardCellType {
    Empty,
    X,
    O,
}

impl BoardCellType {
    fn to_char(&self) -> char {
        match self {
            BoardCellType::Empty => ' ',
            BoardCellType::X => 'X',
            BoardCellType::O => 'O',
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

    pub fn board_cell_types(&self) -> [BoardCellType; 9] {
        let mut cell_types = [BoardCellType::Empty; 9];
        let mut x = true;
        for n in &self.moves {
            if x {
                cell_types[*n as usize] = BoardCellType::X;
            } else {
                cell_types[*n as usize] = BoardCellType::O;
            }
            x = !x;
        }
        cell_types
    }

    fn current_player(&self) -> BoardCellType {
        if self.moves.len() % 2 == 0 {
            BoardCellType::X
        } else {
            BoardCellType::O
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
        let cell_types = self.board_cell_types();

        let mut wins = vec![];
        for win in possible_wins {
            let values = win
                .iter()
                .map(|n| cell_types[*n as usize])
                .collect::<Vec<_>>();
            if values.iter().all(|v| *v == BoardCellType::X) {
                wins.push(win);
            } else if values.iter().all(|v| *v == BoardCellType::O) {
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

    fn winner(&self) -> Option<BoardCellType> {
        let cell_types = self.board_cell_types();
        for win in self.winning_lines() {
            for value in win {
                let cell_type = cell_types[value as usize];
                return Some(cell_type);
            }
        }
        None
    }

    fn message(&self) -> String {
        if let Some(winner) = self.winner() {
            format!("{} wins!", winner.to_char())
        } else if self.moves.len() == 9 {
            "It's a draw!".to_string()
        } else {
            format!("{} to play", self.current_player().to_char())
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
                if self.moves.contains(&move_) {
                    return Command::none();
                }
                if self.winning_lines().is_empty() {
                    self.moves.push(move_);
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
        // Title and message at start of collumn
        let mut column = Column::new()
            .padding(0)
            .spacing(0)
            .align_items(Align::Center)
            .push(Text::new("Tic Tac Toe").size(BASE_SIZE * 10))
            .push(Text::new(self.message()).size(BASE_SIZE * 8));

        // Get 2D array of buttons
        let winning_squares = self.winning_squares();
        let cell_types = self.board_cell_types();
        let mut board_elements = Vec::new();
        let mut row_elements = Vec::new();
        let column_count = 3;
        for (i, state) in self.cell_button_states.iter_mut().enumerate() {
            let cell_type = cell_types[i];
            let winning = winning_squares.contains(&(i as u8));
            row_elements.push(board_element(state, cell_type, i as u8, winning));
            if i % column_count == column_count - 1 {
                board_elements.push(row_elements);
                row_elements = Vec::new();
            }
        }
        if !row_elements.is_empty() {
            board_elements.push(row_elements);
        }

        // Convert 2D array of buttons to a column of rows.
        let rows = board_elements.into_iter().map(|e| board_row(e));
        for row in rows.into_iter() {
            column = column.push(row);
        }

        // Add management row.
        let reset_button = svg_button(
            &mut self.reset_button_state,
            svg_from_file("reset"),
            Message::Reset,
        );
        let undo_button = svg_button(
            &mut self.undo_button_state,
            svg_from_file("undo"),
            Message::Undo,
        );
        let management_row = board_row(vec![reset_button, undo_button]);
        column = column.push(management_row);

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

fn svg_from_file(name: &str) -> Svg {
    Svg::from_path(format!(
        "{}/resources/{}.svg",
        env!("CARGO_MANIFEST_DIR"),
        name
    ))
}

fn svg_button(state: &mut button::State, svg: Svg, message: Message) -> Button<Message> {
    Button::new(state, svg)
        .on_press(message)
        .height(Length::Units(BASE_SIZE * 20))
        .width(Length::Units(BASE_SIZE * 20))
}

fn board_row<'a>(buttons: Vec<Button<'a, Message>>) -> Row<'a, Message> {
    // padding works above and below a row, so it doubles up between rows.  It controls vertical spacing.
    let row_padding = BASE_SIZE;
    // spacing works between row elements, so it does not double up.  It controls horizontal spacing.
    let row_spacing = 2 * row_padding;

    let mut row = Row::new().padding(row_padding).spacing(row_spacing);
    for button in buttons.into_iter() {
        row = row.push(button);
    }
    row
}

fn board_element(
    state: &mut button::State,
    cell_type: BoardCellType,
    index: u8,
    winner: bool,
) -> Button<Message> {
    // Style depends on whether the board_cell is a winning one or not.
    let size;
    let color;
    if winner {
        size = BASE_SIZE * 18;
        color = iced::Color::from_rgb(0.5, 0.5, 0.5);
    } else {
        size = BASE_SIZE * 16;
        color = iced::Color::from_rgb(0.8, 0.8, 0.8);
    };

    // Pick and style text
    let text = Text::new(cell_type.to_char().to_string())
        .height(Length::Fill)
        .width(Length::Fill)
        .horizontal_alignment(iced::HorizontalAlignment::Center)
        .vertical_alignment(iced::VerticalAlignment::Center)
        .size(size);

    // Style the button
    Button::new(state, text)
        .style(ButtonColor { color })
        .on_press(Message::MoveMade(index))
        .padding(BASE_SIZE * 2)
        .height(Length::Units(BASE_SIZE * 20))
        .width(Length::Units(BASE_SIZE * 20))
}
