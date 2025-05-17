use crate::bag::Bag;
use crate::colors::WHITE_COLOR;
use crate::constants::{OFFSET_Y, PLAYGROUND_HEIGHT, PLAYGROUND_WIDTH, SPACING, SQUARE_SIZE};
use crate::shapes::{Matrix, Shape};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::canvas;
use iced::widget::canvas::{Cache, Geometry, Path, Stroke, Text};
use iced::{Color, Pixels, Point, Rectangle, Renderer, Size, Theme, mouse};

#[derive(Debug, Default)]
pub struct State {
    pub now: chrono::DateTime<chrono::Local>,
    pub playground: Cache,
    pub bag: Bag,
    pub game_space: Matrix,
    pub tick_rate_ms: u64,
    pub score: u32,
    pub is_running: bool,
    pub next_item: Shape,
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
            let offset_x = half - PLAYGROUND_WIDTH / 2.0;

            frame.fill_text(Text {
                content: format!("Score: {}", self.score),
                position: Point {
                    x: half + PLAYGROUND_WIDTH / 2.0 + 10.0,
                    y: OFFSET_Y,
                },
                color: WHITE_COLOR.into(),
                size: Pixels(14.0),
                horizontal_alignment: Horizontal::Left,
                vertical_alignment: Vertical::Center,
                ..Default::default()
            });

            frame.fill_text(Text {
                content: "Next brick".to_string(),
                position: Point {
                    x: half + PLAYGROUND_WIDTH / 2.0 + 10.0,
                    y: OFFSET_Y + 20.0,
                },
                color: WHITE_COLOR.into(),
                size: Pixels(14.0),
                horizontal_alignment: Horizontal::Left,
                vertical_alignment: Vertical::Center,
                ..Default::default()
            });

            let offset = 30.0;
            let mut seen = false;
            let mut row_index = 0;

            for (_, row) in self.next_item.matrix.iter().enumerate() {
                seen = false;
                
                for (col_index, item) in row.iter().enumerate() {
                    if let Some(brick) = item {
                        seen = true;
                        
                        let rect = Path::rectangle(
                            Point {
                                x: col_index as f32 * SQUARE_SIZE
                                    + half
                                    + PLAYGROUND_WIDTH / 2.0
                                    + 10.0
                                    + col_index as f32 * SPACING,
                                y: row_index as f32 * SQUARE_SIZE
                                    + OFFSET_Y
                                    + offset
                                    + row_index as f32 * SPACING,
                            },
                            Size {
                                width: SQUARE_SIZE,
                                height: SQUARE_SIZE,
                            },
                        );

                        frame.fill(&rect, brick.color);
                    }
                }
                
                if seen {
                    row_index += 1;
                }
            }

            if !self.is_running {
                frame.fill_rectangle(
                    Point::new(half - 75.0, half - OFFSET_Y - 25.0),
                    Size {
                        width: 150.0,
                        height: 50.0,
                    },
                    Color::from_rgb(0.5, 0.2, 0.8),
                );

                frame.fill_text(Text {
                    content: "Press space to start".into(),
                    position: Point {
                        x: half - 55.0,
                        y: half - OFFSET_Y,
                    },
                    color: WHITE_COLOR.into(),
                    size: Pixels(14.0),
                    horizontal_alignment: Horizontal::Left,
                    vertical_alignment: Vertical::Center,
                    ..Default::default()
                });
            }

            for (row_index, row) in self.game_space.iter().enumerate() {
                for (col_index, cell) in row.iter().enumerate() {
                    if let Some(brick) = cell {
                        let x = offset_x + col_index as f32 * (SQUARE_SIZE + SPACING);
                        let y = OFFSET_Y + row_index as f32 * (SQUARE_SIZE + SPACING);

                        let rect = Path::rectangle(
                            Point { x, y },
                            Size {
                                width: SQUARE_SIZE,
                                height: SQUARE_SIZE,
                            },
                        );

                        frame.fill(&rect, brick.color);
                    }
                }
            }

            let start_line = Path::line(
                Point {
                    x: half - PLAYGROUND_WIDTH / 2.0,
                    y: OFFSET_Y,
                },
                Point {
                    x: half - PLAYGROUND_WIDTH / 2.0,
                    y: PLAYGROUND_HEIGHT + OFFSET_Y,
                },
            );

            let end_line = Path::line(
                Point {
                    x: half + PLAYGROUND_WIDTH / 2.0,
                    y: OFFSET_Y,
                },
                Point {
                    x: half + PLAYGROUND_WIDTH / 2.0,
                    y: PLAYGROUND_HEIGHT + OFFSET_Y,
                },
            );

            let bottom_line = Path::line(
                Point {
                    x: half - PLAYGROUND_WIDTH / 2.0,
                    y: PLAYGROUND_HEIGHT + OFFSET_Y,
                },
                Point {
                    x: half + PLAYGROUND_WIDTH / 2.0,
                    y: PLAYGROUND_HEIGHT + OFFSET_Y,
                },
            );

            frame.stroke(
                &bottom_line,
                Stroke::default().with_color(WHITE_COLOR.into()),
            );

            frame.stroke(
                &start_line,
                Stroke::default().with_color(WHITE_COLOR.into()),
            );

            frame.stroke(&end_line, Stroke::default().with_color(WHITE_COLOR.into()));
        });
        vec![playground]
    }
}
