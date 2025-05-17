use crate::bag::Bag;
use crate::colors::WHITE_COLOR;
use crate::constants::{OFFSET_Y, PLAYGROUND_HEIGHT, PLAYGROUND_WIDTH, SPACING, SQUARE_SIZE};
use crate::shapes::Matrix;
use iced::widget::canvas;
use iced::widget::canvas::{Cache, Geometry, Path, Stroke, Text};
use iced::{Point, Rectangle, Renderer, Size, Theme, mouse, Pixels};
use iced::alignment::{Horizontal, Vertical};

#[derive(Debug, Default)]
pub struct State {
    pub now: chrono::DateTime<chrono::Local>,
    pub playground: Cache,
    pub bag: Bag,
    pub game_space: Matrix,
    pub tick_rate_ms: u64,
    pub score: u32,
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
            
            frame.fill_text(Text{
                content: format!("Score: {}", self.score),
                position: Point {
                    x: half + PLAYGROUND_WIDTH / 2.0 + 10.0,
                    y: OFFSET_Y,
                },
                color: WHITE_COLOR.into(),
                size: Pixels(14.0),
                line_height: Default::default(),
                font: Default::default(),
                horizontal_alignment: Horizontal::Left,
                vertical_alignment: Vertical::Center,
                shaping: Default::default(),
            });

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
