use crate::bag::Bag;
use crate::canvas::State;
use crate::enums::Direction;
use crate::moves::{move_bottom, move_left, move_right};
use crate::rotations::{rotate_clockwise, rotate_counterclockwise};
use crate::shapes::Shape;
use iced::Task;
use crate::level::{get_level, get_speed_by_level};
use crate::playground::Playground;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Initialize,
    Tick(chrono::DateTime<chrono::Local>),
    Move(Direction),
    RotateClockwise,
    Rotate,
    TogglePause,
}

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::TogglePause => {
            state.is_running = !state.is_running;
            Task::none()
        }
        Message::Move(dir) => {
            if !state.is_running {
                return Task::none();
            }

            if dir == Direction::Right {
                move_right(&mut state.game_space);
            } else if dir == Direction::Left {
                move_left(&mut state.game_space);
            } else {
                move_bottom(&mut state.game_space, &mut state.bag, &mut state.next_item);

                let cleared_rows = Playground::clear_rows(&mut state.game_space, &mut state.score);
                state.rows_cleared += cleared_rows;
                state.level = get_level(state.rows_cleared);
                state.tick_rate_ms = get_speed_by_level(state.level);
            }

            state.playground.clear();
            Task::none()
        }
        Message::Initialize => {
            let mut bag = Bag::new();
            bag.default_items = vec![
                Shape::create_o(),
                Shape::create_i(),
                Shape::create_s(),
                Shape::create_z(),
                Shape::create_t(),
                Shape::create_l(),
                Shape::create_j(),
            ];
            bag.refill();
            state.bag = bag;

            let mut item = state.bag.get_item();
            if let Some(next_item) = state.bag.show_next() {
                state.next_item = next_item;
            }

            item.set_default_position(&mut state.game_space);

            Task::none()
        }
        Message::Tick(local_time) => {
            let now = local_time;

            if now != state.now {
                state.now = now;
                state.playground.clear();
            }

            Task::perform(async {}, |_| Message::Move(Direction::Bottom))
        }
        Message::RotateClockwise => {
            rotate_clockwise(&mut state.game_space);
            state.playground.clear();
            Task::none()
        }

        Message::Rotate => {
            rotate_counterclockwise(&mut state.game_space);
            state.playground.clear();
            Task::none()
        }
    }
}
