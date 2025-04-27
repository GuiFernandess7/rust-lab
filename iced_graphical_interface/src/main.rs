use iced::{Alignment, Command, Element, Sandbox, Settings};
use ::iced::theme::Theme;
use iced::widget::Text;

pub fn main() -> iced::Result {
    SimpleApp::run(Settings::default())
}

struct SimpleApp {
    theme: Theme,
    page: Page,
    login_field: LoginField
}

struct LoginField {email: String, password: String}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Page {Login, Register}

#[derive(Debug, Clone)]
enum Message {
    ToggleTheme,
    LoginSubmit,
    Router(String),
    LoginFieldChange(String, String),
}

impl Sandbox for SimpleApp {
    type Message = Message;

    fn new() -> Self {
        Self {
            theme: Theme::Dark,
            page: Page::Login,
            login_field: LoginField {
                email: String::new(),
                password: String::new(),
            }
        }
    }

    fn title(&self) -> String {
        String::from("Rust UI Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleTheme => {}
            Message::LoginFieldChange(email, password) => {}
            Message::LoginSubmit => {}
            Message::Router(route) => {}
        }
    }

    fn view(&self) -> Element<Message> {
        Text::new("Hello, World").into()
    }
}
