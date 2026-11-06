use iced::Event::Mouse;
use iced::Length::Fill;
use iced::mouse::ScrollDelta;
use iced::widget::{button, column, container, text};
use iced::window::Position;
use iced::{Element, Point, Size, Subscription, Theme, event, mouse};

pub struct WordMap {
    pos: Point,
    zoom: f32,
}

impl Default for WordMap {
    fn default() -> Self {
        Self {
            pos: Point { x: 0.0, y: 0.0 },
            zoom: 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ChangeZoom(f32),
}

impl WordMap {
    pub fn view(&self) -> Element<'_, Message> {
        container(column![
            button("+").on_press(Message::ChangeZoom(0.05)),
            text(self.zoom).size(50),
            button("-").on_press(Message::ChangeZoom(-0.05)),
            text("Hello!"),
        ])
        .center_x(Fill)
        .center_y(Fill)
        .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            /*Message::ZoomIn => {
                self.zoom *= 2.0;
            }
            Message::ZoomOut => {
                self.zoom /= 2.0;
            }*/
            Message::ChangeZoom(value) => {
                self.zoom += value;
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, status, id| match event {
            event::Event::Mouse(event) => match event {
                mouse::Event::WheelScrolled {
                    delta: ScrollDelta::Lines { x, y },
                } => Some(Message::ChangeZoom(y)),
                _ => None,
            },
            _ => None,
        })
    }
}

pub fn launch() -> iced::Result {
    iced::application(WordMap::default, WordMap::update, WordMap::view)
        .theme(|state: &WordMap| Theme::Dracula)
        //.position(Position::SpecificWith(position))
        .scale_factor(|state| state.zoom)
        .subscription(WordMap::subscription)
        .run()
}

fn position(window_size: Size, resolution: Size) -> Point {
    Point { x: 0.0, y: 0.0 }
}
