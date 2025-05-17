use iced::{Element, Fill};
use iced::widget::{canvas, container};
use crate::canvas::State;
use crate::update::Message;

pub fn view(state: &State) -> Element<Message> {
    let canvas = canvas(state).width(Fill).height(Fill);
    container(canvas).into()
}