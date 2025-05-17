use iced::Task;
use crate::bag::Bag;
use crate::canvas::State;
use crate::enums::Direction;
use crate::shapes::Shape;
use crate::rotations::{rotate_clockwise, rotate_counterclockwise};
use crate::score::clear_rows;

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
                let rows = state.game_space.len();
                let cols = state.game_space[0].len();

                let mut can_move = true;

                for row in 0..rows {
                    for col in 0..cols {
                        if let Some(brick) = &state.game_space[row][col] {
                            if brick.moving {
                                if col + 1 >= cols
                                    || (state.game_space[row][col + 1].is_some()
                                    && !state.game_space[row][col + 1].as_ref().unwrap().moving)
                                {
                                    can_move = false;
                                    break;
                                }
                            }
                        }
                    }
                    if !can_move {
                        break;
                    }
                }

                if can_move {
                    for row in 0..rows {
                        for col in (0..cols - 1).rev() {
                            if let Some(brick) = &state.game_space[row][col] {
                                if brick.moving {
                                    state.game_space[row][col + 1] =
                                        state.game_space[row][col].take();
                                }
                            }
                        }
                    }
                }
            } else if dir == Direction::Left {
                let rows = state.game_space.len();
                let cols = state.game_space[0].len();

                let mut can_move = true;

                for row in 0..rows {
                    for col in 0..cols {
                        if let Some(brick) = &state.game_space[row][col] {
                            if brick.moving {
                                if col == 0
                                    || (state.game_space[row][col - 1].is_some()
                                    && !state.game_space[row][col - 1].as_ref().unwrap().moving)
                                {
                                    can_move = false;
                                    break;
                                }
                            }
                        }
                    }
                    if !can_move {
                        break;
                    }
                }

                if can_move {
                    for row in 0..rows {
                        for col in 1..cols {
                            if let Some(brick) = &state.game_space[row][col] {
                                if brick.moving {
                                    state.game_space[row][col - 1] =
                                        state.game_space[row][col].take();
                                }
                            }
                        }
                    }
                }
            } else {
                let mut can_move = true;
                let rows = state.game_space.len();
                let cols = state.game_space[0].len();

                for row in (0..rows).rev() {
                    for col in 0..cols {
                        if let Some(brick) = &state.game_space[row][col] {
                            if brick.moving {
                                if row + 1 >= rows
                                    || (state.game_space[row + 1][col].is_some()
                                    && !state.game_space[row + 1][col].as_ref().unwrap().moving)
                                {
                                    can_move = false;
                                    break;
                                }
                            }
                        }
                    }
                }

                if can_move {
                    for row in (0..rows - 1).rev() {
                        for col in 0..cols {
                            if let Some(brick) = &state.game_space[row][col] {
                                if brick.moving {
                                    state.game_space[row + 1][col] =
                                        state.game_space[row][col].take();
                                }
                            }
                        }
                    }

                    for col in 0..cols {
                        state.game_space[0][col] = None;
                    }
                } else {
                    let rows = state.game_space.len();
                    let cols = state.game_space[0].len();

                    for row in 0..rows {
                        for col in 0..cols {
                            if let Some(brick) = &mut state.game_space[row][col] {
                                if brick.moving {
                                    brick.moving = false;
                                }
                            }
                        }
                    }

                    let item = state.bag.get_item();
                    if let Some(next_item) = state.bag.show_next() {
                        state.next_item = next_item;
                    }
                    
                    let mut seen = false;
                    let mut start_row = 0;
                    let start_col = state.game_space[0].len() / 2 - 1;

                    for (_, row) in item.matrix.iter().enumerate() {
                        seen = false;

                        for (col_index, cell) in row.iter().enumerate() {
                            if let Some(brick) = cell {
                                seen = true;
                                let x = start_row;
                                let y = start_col + col_index;

                                let mut new_brick = brick.clone();

                                new_brick.moving = true;
                                state.game_space[x][y] = Some(new_brick);
                            }
                        }

                        if seen {
                            start_row += 1
                        }
                    }
                }

                clear_rows(&mut state.game_space, &mut state.score);
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

            let item = state.bag.get_item();
            if let Some(next_item) = state.bag.show_next() {
                state.next_item = next_item;
            }
            
            let mut seen = false;
            let mut start_row = 0;
            let start_col = state.game_space[0].len() / 2 - 1;

            for (_, row) in item.matrix.iter().enumerate() {
                seen = false;

                for (col_index, cell) in row.iter().enumerate() {
                    if let Some(brick) = cell {
                        seen = true;
                        let x = start_row;
                        let y = start_col + col_index;

                        let mut new_brick = brick.clone();

                        if x < state.game_space.len() && y < state.game_space[0].len() {
                            new_brick.moving = true;
                            state.game_space[x][y] = Some(new_brick);
                        }
                    }
                }

                if seen {
                    start_row += 1
                }
            }

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