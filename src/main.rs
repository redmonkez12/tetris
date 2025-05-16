mod bag;
mod shapes;
mod utils;

use crate::bag::Bag;
use crate::shapes::{Brick, Matrix, Shape};
use crate::utils::{rotate_clockwise, rotate_counterclockwise};
use iced::keyboard;
use iced::keyboard::Key::Named;
use iced::theme::{Custom, Palette};
use iced::widget::canvas::{Cache, Geometry, Path, Stroke};
use iced::widget::{canvas, container};
use iced::{
    Color, Element, Fill, Point, Rectangle, Renderer, Size, Subscription, Task, Theme, mouse,
};
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Bottom,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Initialize,
    Tick(chrono::DateTime<chrono::Local>),
    Move(Direction),
    RotateClockwise,
    Rotate,
}

#[derive(Debug, Default)]
struct State {
    now: chrono::DateTime<chrono::Local>,
    playground: Cache,
    bag: Bag,
    tick_rate_ms: u64,
    game_space: Vec<Vec<Option<Brick>>>,
}

impl<Message> canvas::Program<Message> for State {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let playground = self.playground.draw(renderer, bounds.size(), |frame| {
            let half = bounds.width / 2.0;

            let square_size = 20.0;
            let num_of_squares_x = 10.0;
            let num_of_squares_y = 20.0;
            let spacing = 1.0;
            let width = num_of_squares_x * square_size + num_of_squares_x * spacing;
            let height = num_of_squares_y * square_size + num_of_squares_y * spacing;
            let offset_y = 150.0;
            let offset_x = half - width / 2.0;

            for (row_index, row) in self.game_space.iter().enumerate() {
                for (col_index, cell) in row.iter().enumerate() {
                    if let Some(brick) = cell {
                        let x = col_index as f32 * (square_size + spacing) + offset_x;
                        let y = row_index as f32 * (square_size + spacing) + offset_y;

                        println!("x: {}, y: {}", x, y);

                        let rect = Path::rectangle(
                            Point { x, y },
                            Size {
                                width: square_size,
                                height: square_size,
                            },
                        );

                        frame.fill(&rect, brick.color);
                    }
                }
            }

            let start_line = Path::line(
                Point {
                    x: half - width / 2.0,
                    y: offset_y,
                },
                Point {
                    x: half - width / 2.0,
                    y: height + offset_y,
                },
            );

            let end_line = Path::line(
                Point {
                    x: half + width / 2.0,
                    y: offset_y,
                },
                Point {
                    x: half + width / 2.0,
                    y: height + offset_y,
                },
            );

            let bottom_line = Path::line(
                Point {
                    x: half - width / 2.0,
                    y: height + offset_y,
                },
                Point {
                    x: half + width / 2.0,
                    y: height + offset_y,
                },
            );

            frame.stroke(
                &bottom_line,
                Stroke::default().with_color([0.976, 0.980, 0.984].into()),
            );

            frame.stroke(
                &start_line,
                Stroke::default().with_color([0.976, 0.980, 0.984].into()),
            );

            frame.stroke(
                &end_line,
                Stroke::default().with_color([0.976, 0.980, 0.984].into()),
            );
        });
        vec![playground]
    }
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Move(dir) => {
            if dir == Direction::Right {
                let rows = state.game_space.len();
                let cols = state.game_space[0].len();

                for row in 0..rows - 1 {
                    for col in (1..cols).rev() {
                        let brick = state.game_space[row][col - 1];

                        if brick.is_some() && brick.unwrap().moving {
                            state.game_space[row][col] = state.game_space[row][col - 1].take();
                        }
                    }

                    state.game_space[row][0] = None;
                }
            } else if dir == Direction::Left {
                let rows = state.game_space.len();
                let cols = state.game_space[0].len();

                for row in 0..rows {
                    for col in 0..cols - 1 {
                        let brick = state.game_space[row][col + 1];

                        if brick.is_some() && brick.unwrap().moving {
                            state.game_space[row][col] = state.game_space[row][col + 1].take();
                        }
                    }
                    state.game_space[row][cols - 1] = None;
                }
            } else {
                let rows = state.game_space.len();
                let cols = state.game_space[0].len();

                for row in (0..rows - 1).rev() {
                    for col in 0..cols {
                        let brick = state.game_space[row][col];

                        if brick.is_some() && brick.unwrap().moving {
                            state.game_space[row + 1][col] = state.game_space[row][col].take();
                        }
                    }
                }

                for col in 0..cols {
                    state.game_space[0][col] = None;
                }
            }
            state.playground.clear();

            Task::none()
        }
        Message::Initialize => {
            println!("Initialize");

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
            let start_row = 0;
            let start_col = state.game_space[0].len() / 2 - 1;

            for (row_index, row) in item.matrix.iter().enumerate() {
                let mut new_row: Vec<Option<Brick>> = vec![];

                for (col_index, cell) in row.iter().enumerate() {
                    if let Some(brick) = cell {
                        let x = start_row + row_index;
                        let y = start_col + col_index;

                        let mut new_brick = brick.clone();
                        new_brick.moving = true;

                        if x < state.game_space.len() && y < state.game_space[0].len() {
                            state.game_space[x][y] = Some(new_brick);
                        }

                        new_row.push(Some(new_brick));
                    } else {
                        new_row.push(None);
                    }
                }
            }

            Task::none()
        }
        Message::Tick(local_time) => {
            let now = local_time;

            if now != state.now {
                state.now = now;
                let rows = state.game_space.len();
                let cols = state.game_space[0].len();

                let mut can_move = true;

                for row in (0..rows - 1).rev() {
                    for col in 0..cols {
                        if let Some(brick) = &state.game_space[row][col] {
                            if brick.moving {
                                if row + 1 >= rows || state.game_space[row + 1][col].is_some() {
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
                                    state.game_space[row + 1][col] = state.game_space[row][col].take();
                                }
                            }
                        }
                    }

                    for col in 0..cols {
                        state.game_space[0][col] = None;
                    }
                } else {
                    for row in 0..rows {
                        for col in 0..cols {
                            if let Some(brick) = &mut state.game_space[row][col] {
                                if brick.moving {
                                    brick.moving = false;
                                }
                            }
                        }
                    }
                }

                state.playground.clear();
            }

            Task::none()
        }
        Message::RotateClockwise => {
            // if let Some(mut current_item) = detect_current_shape(&state.game_space) {
            //     clear_moving_bricks(&mut state.game_space);
            //     rotate_clockwise(&mut current_item);
            //     draw_shape_into_game_space(&current_item, &mut state.game_space);
            //     state.playground.clear();
            // }
            Task::none()
        }

        Message::Rotate => {
            // if let Some(mut current_item) = detect_current_shape(&state.game_space) {
            //     println!("{:?}", current_item);
            //     clear_moving_bricks(&mut state.game_space);
            //     rotate_counterclockwise(&mut current_item);
            //     draw_shape_into_game_space(&current_item, &mut state.game_space);
            //     state.playground.clear();
            // }
            Task::none()
        }
    }
}

fn view(state: &State) -> Element<Message> {
    let canvas = canvas(state).width(Fill).height(Fill);
    container(canvas).into()
}

fn subscription(state: &State) -> Subscription<Message> {
    fn handle_hotkey(key: keyboard::Key, _modifiers: keyboard::Modifiers) -> Option<Message> {
        println!("{:?}", key.as_ref().clone());
        match key.as_ref() {
            keyboard::Key::Character("q") => Some(Message::RotateClockwise),
            keyboard::Key::Character("w") => Some(Message::Rotate),
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
        tick_rate_ms: 1000,
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
