mod bag;
mod shapes;
mod rotations;
mod view;
mod update;
mod subscription;
mod canvas;
mod enums;
mod colors;
mod constants;
mod score;

use crate::bag::Bag;
use crate::shapes::{Brick, Matrix, Shape};
use iced::theme::{Custom, Palette};
use iced::widget::canvas::{Cache};
use iced::{
    Color, Task, Theme,
};
use std::sync::Arc;
use crate::canvas::State;
use crate::subscription::subscription;
use crate::update::{update, Message};
use crate::view::view;

fn init() -> (State, Task<Message>) {
    tracing_subscriber::fmt::init();

    let num_of_squares_x = 10;
    let num_of_squares_y = 20;
    let mut game_space: Vec<Vec<Option<Brick>>> = Vec::new();

    for _ in 0..num_of_squares_y {
        game_space.push(vec![None; num_of_squares_x]);
    }

    let state = State {
        playground: Cache::new(),
        bag: Bag::new(),
        game_space,
        tick_rate_ms: 500,
        is_running: false,
        ..Default::default()
    };
    (state, Task::perform(async {}, |_| Message::Initialize))
}

fn theme(_: &State) -> Theme {
    let custom_theme = Arc::new(Custom::new(
        "My Dark Theme".into(),
        Palette {
            background: [0.012, 0.027, 0.071].into(),
            text: [0.976, 0.980, 0.984].into(),
            primary: Color::from_rgb(0.3, 0.6, 0.9),
            success: Color::from_rgb(0.2, 0.8, 0.4),
            danger: Color::from_rgb(0.9, 0.2, 0.2),
        },
    ));

    Theme::Custom(custom_theme)
}

fn main() -> iced::Result {
    iced::application("Tetris", update, view)
        .theme(theme)
        .subscription(subscription)
        .run_with(init)
}
