mod bag;
mod canvas;
mod colors;
mod constants;
mod enums;
mod level;
mod moves;
mod playground;
mod rotations;
mod shapes;
mod subscription;
mod types;
mod update;
mod view;

use crate::bag::Bag;
use crate::canvas::State;
use crate::colors::{BLACK_COLOR, DANGER_COLOR, PRIMARY_COLOR, SECONDARY_COLOR, WHITE_COLOR};
use crate::subscription::subscription;
use crate::update::{Message, update};
use crate::view::view;
use iced::theme::{Custom, Palette};
use iced::widget::canvas::Cache;
use iced::{Task, Theme};
use std::sync::Arc;
use crate::constants::DEFAULT_LEVEL;
use crate::level::get_speed_by_level;

fn init() -> (State, Task<Message>) {
    tracing_subscriber::fmt::init();

    let state = State {
        playground: Cache::new(),
        bag: Bag::new(),
        game_space: Vec::new(),
        tick_rate_ms: get_speed_by_level(DEFAULT_LEVEL),
        is_running: false,
        level: DEFAULT_LEVEL,
        rows_cleared: 0,
        ..Default::default()
    };
    (state, Task::perform(async {}, |_| Message::Initialize))
}

fn theme(_: &State) -> Theme {
    let custom_theme = Arc::new(Custom::new(
        "My Dark Theme".into(),
        Palette {
            background: BLACK_COLOR.into(),
            text: WHITE_COLOR.into(),
            primary: PRIMARY_COLOR.into(),
            success: SECONDARY_COLOR.into(),
            danger: DANGER_COLOR.into(),
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
