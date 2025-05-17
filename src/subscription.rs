use std::time::Duration;
use iced::{keyboard, Subscription};
use iced::keyboard::Key::Named;
use crate::canvas::State;
use crate::enums::Direction;
use crate::update::Message;

pub fn subscription(state: &State) -> Subscription<Message> {
    fn handle_hotkey(key: keyboard::Key, _modifiers: keyboard::Modifiers) -> Option<Message> {
        println!("{:?}", key.as_ref().clone());
        match key.as_ref() {
            keyboard::Key::Character("q") => Some(Message::RotateClockwise),
            keyboard::Key::Character("w") => Some(Message::Rotate),
            Named(keyboard::key::Named::Space) => Some(Message::TogglePause),
            Named(keyboard::key::Named::ArrowLeft) => Some(Message::Move(Direction::Left)),
            Named(keyboard::key::Named::ArrowRight) => Some(Message::Move(Direction::Right)),
            Named(keyboard::key::Named::ArrowDown) => Some(Message::Move(Direction::Bottom)),
            _ => None,
        }
    }

    Subscription::batch([
        iced::time::every(Duration::from_millis(state.tick_rate_ms))
            .map(|_| Message::Tick(chrono::offset::Local::now())),
        keyboard::on_key_press(handle_hotkey),
    ])
}
