use crate::bag::Bag;
use crate::canvas::State;
use crate::constants::{NUM_OF_SQUARES_X, NUM_OF_SQUARES_Y};
use crate::enums::Direction;
use crate::level::{get_level, get_speed_by_level};
use crate::moves::{draw_game_over_brick, is_game_over, move_bottom, move_left, move_right};
use crate::playground::Playground;
use crate::rotations::{rotate_clockwise, rotate_counterclockwise};
use crate::types::{Matrix, TimeLocal};
use iced::Task;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Initialize,
    Tick(TimeLocal),
    Move(Direction),
    RotateClockwise,
    Rotate,
    TogglePause,
}

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::TogglePause => {
            if state.game_over {
                state.game_space = state.default_game_space.clone();
                state.game_over = false;
                state.bag.refill();
                state.score = 0;
                state.rows_cleared = 0;
                state.level = 0;
                state.tick_rate_ms = get_speed_by_level(state.level);

                let (mut item, next_item) = state.bag.get_item();
                state.next_item = next_item;

                item.set_default_position(&mut state.game_space);
            }

            state.is_running = !state.is_running;
            state.playground.clear();
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
                let moved = move_bottom(&mut state.game_space);
                
                if !moved {
                    let (mut item, next_item) = state.bag.get_item();

                    let (game_over, rows_to_render) = is_game_over(&mut state.game_space, &item);
                    
                    if game_over {
                        println!("Game over!");
                        println!("Rows to render: {}", rows_to_render);

                        let x_pos = state.game_space[0].len() / 2 - 1;
                        draw_game_over_brick(&mut state.game_space, x_pos, rows_to_render, &item);
                        
                        state.is_running = false;
                        state.game_over = true;
                        return Task::none();
                    } else {
                        state.next_item = next_item.clone();
                        item.set_default_position(&mut state.game_space);
                        
                        let cleared_rows = Playground::clear_rows(&mut state.game_space, &mut state.score);
                        state.rows_cleared += cleared_rows;
                        state.level = get_level(state.rows_cleared);
                        state.tick_rate_ms = get_speed_by_level(state.level);
                    }
                }
            }

            state.playground.clear();
            Task::none()
        }
        Message::Initialize => {
            let mut game_space: Matrix = Vec::new();

            for _ in 0..NUM_OF_SQUARES_Y as usize {
                game_space.push(vec![None; NUM_OF_SQUARES_X as usize]);
            }

            state.game_space = game_space.clone();
            state.default_game_space = game_space;

            let mut bag = Bag::new();
            bag.refill();
            state.bag = bag;

            let (mut item, next_item) = state.bag.get_item();
            state.next_item = next_item;

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
            if state.is_running {
                rotate_clockwise(&mut state.game_space);
                state.playground.clear();
            }

            Task::none()
        }

        Message::Rotate => {
            if state.is_running {
                rotate_counterclockwise(&mut state.game_space);
                state.playground.clear();
            }

            Task::none()
        }
    }
}
