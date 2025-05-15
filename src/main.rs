mod bag;
mod shapes;

use crate::bag::Bag;
use crate::shapes::Shape;
use iced::widget::canvas::{Cache, Geometry, Path, Stroke};
use iced::widget::{canvas, container};
use iced::{
    Color, Element, Fill, Point, Rectangle, Renderer, Size, Subscription, Task, Theme, mouse,
};
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
enum Message {
    Initialize,
    Tick(chrono::DateTime<chrono::Local>),
}

#[derive(Default)]
struct State {
    now: chrono::DateTime<chrono::Local>,
    playground: Cache,
    bag: Bag,
    tick_rate_ms: u64,
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
            println!("empty: {}", self.bag.items.is_empty());

            if !self.bag.items.is_empty() {
                let item = self.bag.items.get(0).unwrap();

                let x0 = half - square_size;
                let y0 = half - square_size;

                for (row_index, row) in item.iter().enumerate() {
                    for (col_index, &cell) in row.iter().enumerate() {
                        if cell == 1 {
                            let x = x0 + col_index as f32 * square_size;
                            let y = y0 + row_index as f32 * square_size;

                            println!("x: {}, y: {}", x, y);

                            let rect = Path::rectangle(
                                Point { x, y },
                                Size {
                                    width: square_size,
                                    height: square_size,
                                },
                            );

                            frame.fill(
                                &rect,
                                Color::from_rgb(66.0 / 255.0, 149.0 / 255.0, 137.0 / 255.0),
                            );
                        }
                    }
                }
            }

            let start_line = Path::line(
                Point {
                    x: half - 200.0,
                    y: 0.0 + 50.0,
                },
                Point {
                    x: half - 200.0,
                    y: bounds.height - 50.0,
                },
            );

            let end_line = Path::line(
                Point {
                    x: half + 200.0,
                    y: 0.0 + 50.0,
                },
                Point {
                    x: half + 200.0,
                    y: bounds.height - 50.0,
                },
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

            Task::none()
        }
        Message::Tick(local_time) => {
            let now = local_time;

            if now != state.now {
                state.now = now;
                state.playground.clear();
            }
            
            Task::none()
        }
    }
}

fn view(state: &State) -> Element<Message> {
    let canvas = canvas(state).width(Fill).height(Fill);
    container(canvas).into()
}

fn subscription(state: &State) -> Subscription<Message> {
    iced::time::every(Duration::from_millis(state.tick_rate_ms))
        .map(|_| Message::Tick(chrono::offset::Local::now()))
}

fn init() -> (State, Task<Message>) {
    tracing_subscriber::fmt::init();

    let state = State {
        playground: Cache::new(),
        bag: Bag::new(),
        tick_rate_ms: 1000,
        ..Default::default()
    };
    (state, Task::perform(async {}, |_| Message::Initialize))
}

fn main() -> iced::Result {
    iced::application("Tetris", update, view)
        .subscription(subscription)
        .run_with(init)
}
