#[derive(Default)]
pub struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

use iced::Element;
use iced::Length::Fill;
use iced::widget::{button, column, container, text};

impl Counter {
    pub fn view(&self) -> Element<'_, Message> {
        container(column![
            button("+").on_press(Message::Increment),
            text(self.value).size(50),
            button("-").on_press(Message::Decrement),
            text("Hello!"),
        ])
        .center_x(Fill)
        .center_y(Fill)
        .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }
}
