use iced::{executor, Application, Clipboard, Command, Element, Settings, Text};
use iced::Column;
use iced::Row;

pub fn main() -> iced::Result {
    Hello::run(Settings::default())
}

struct Hello;

impl Application for Hello {
    type Executor = executor::Default;
    type Message = ();
    type Flags = ();

    fn new(_flags: ()) -> (Hello, Command<Self::Message>) {
        (Hello, Command::none())
    }

    fn title(&self) -> String {
        String::from("TicTacToe in Iced")
    }

    fn update(&mut self, _message: Self::Message, _clipboard: &mut Clipboard) -> Command<Self::Message> {
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