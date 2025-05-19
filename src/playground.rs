use iced::{Color, Pixels, Point, Renderer, Size};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::canvas::{Frame, Path, Stroke, Text};
use crate::colors::WHITE_COLOR;
use crate::constants::{OFFSET_Y, PLAYGROUND_HEIGHT, PLAYGROUND_WIDTH, SPACING, SQUARE_SIZE};
use crate::shapes::{Shape};
use crate::types::Matrix;

pub struct Playground {
    half_width: f32,
}

impl Playground {
    pub fn new(half_width: f32) -> Self {
        Self {
            half_width,
        }
    }
    
    pub fn render_game_over(&self, frame: &mut Frame<Renderer>) {
        frame.fill_text(Text {
            content: "Game over".into(),
            position: Point {
                x: self.half_width - 50.0,
                y: self.half_width - OFFSET_Y - 100.0,
            },
            color: WHITE_COLOR.into(),
            size: Pixels(20.0),
            horizontal_alignment: Horizontal::Left,
            vertical_alignment: Vertical::Center,
            ..Default::default()
        });
    }

    pub fn render_level(&mut self, frame: &mut Frame<Renderer>, level: u32) {
        frame.fill_text(Text {
            content: format!("Level: {}", level),
            position: Point {
                x: self.half_width + PLAYGROUND_WIDTH / 2.0 + 10.0,
                y: OFFSET_Y - 20.0,
            },
            color: WHITE_COLOR.into(),
            size: Pixels(14.0),
            horizontal_alignment: Horizontal::Left,
            vertical_alignment: Vertical::Center,
            ..Default::default()
        });
    }

    pub fn render_score(&mut self, frame: &mut Frame<Renderer>, score: u32) {
        frame.fill_text(Text {
            content: format!("Score: {}", score),
            position: Point {
                x: self.half_width + PLAYGROUND_WIDTH / 2.0 + 10.0,
                y: OFFSET_Y,
            },
            color: WHITE_COLOR.into(),
            size: Pixels(14.0),
            horizontal_alignment: Horizontal::Left,
            vertical_alignment: Vertical::Center,
            ..Default::default()
        });
    }

    pub fn render_next_brick(&self, frame: &mut Frame<Renderer>, next_item: &Shape) {
        frame.fill_text(Text {
            content: "Next brick".to_string(),
            position: Point {
                x: self.half_width + PLAYGROUND_WIDTH / 2.0 + 10.0,
                y: OFFSET_Y + 20.0,
            },
            color: WHITE_COLOR.into(),
            size: Pixels(14.0),
            horizontal_alignment: Horizontal::Left,
            vertical_alignment: Vertical::Center,
            ..Default::default()
        });

        let offset = 30.0;
        let mut row_index = 0;

        for (_, row) in next_item.matrix.iter().enumerate() {
            let has_brick = row.iter().any(|cell| cell.is_some());
            
            if !has_brick {
                continue;
            }

            for (col_index, item) in row.iter().enumerate() {
                if let Some(brick) = item {
                    let rect = Path::rectangle(
                        Point {
                            x: col_index as f32 * SQUARE_SIZE
                                + self.half_width
                                + PLAYGROUND_WIDTH / 2.0
                                + 20.0
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

            row_index += 1;
        }
    }

    pub fn render_game_paused(&self, frame: &mut Frame<Renderer>) {
        frame.fill_rectangle(
            Point::new(self.half_width - 75.0, self.half_width - OFFSET_Y - 25.0),
            Size {
                width: 150.0,
                height: 50.0,
            },
            Color::from_rgb(0.5, 0.2, 0.8),
        );

        frame.fill_text(Text {
            content: "Press space to start".into(),
            position: Point {
                x: self.half_width - 60.0,
                y: self.half_width - OFFSET_Y,
            },
            color: WHITE_COLOR.into(),
            size: Pixels(14.0),
            horizontal_alignment: Horizontal::Left,
            vertical_alignment: Vertical::Center,
            ..Default::default()
        });
    }
    
    pub fn render_lines(&self, frame: &mut Frame<Renderer>) {
        let lines = [
            (
                Point {
                    x: self.half_width - PLAYGROUND_WIDTH / 2.0,
                    y: OFFSET_Y,
                },
                Point {
                    x: self.half_width - PLAYGROUND_WIDTH / 2.0,
                    y: PLAYGROUND_HEIGHT + OFFSET_Y,
                },
            ),
            (
                Point {
                    x: self.half_width + PLAYGROUND_WIDTH / 2.0,
                    y: OFFSET_Y,
                },
                Point {
                    x: self.half_width + PLAYGROUND_WIDTH / 2.0,
                    y: PLAYGROUND_HEIGHT + OFFSET_Y,
                },
            ),
            (
                Point {
                    x: self.half_width - PLAYGROUND_WIDTH / 2.0,
                    y: PLAYGROUND_HEIGHT + OFFSET_Y,
                },
                Point {
                    x: self.half_width + PLAYGROUND_WIDTH / 2.0,
                    y: PLAYGROUND_HEIGHT + OFFSET_Y,
                },
            ),
        ];

        for (start, end) in lines {
            let line = Path::line(start, end);
            frame.stroke(&line, Stroke::default().with_color(WHITE_COLOR.into()));
        }
    }
    
    pub fn render_bricks(&self, frame: &mut Frame<Renderer>, game_space: &Matrix) {
        let offset_x = self.half_width - PLAYGROUND_WIDTH / 2.0;

        for (row_index, row) in game_space.iter().enumerate() {
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
    }

    pub fn clear_rows(game_space: &mut Matrix, score: &mut u32) -> u32 {
        let rows = game_space.len();
        let cols = game_space[0].len();

        let mut rows_to_clear = Vec::new();

        for row in 0..rows {
            let mut is_complete = true;

            for col in 0..cols {
                if game_space[row][col].is_none()
                    || (game_space[row][col].is_some() && game_space[row][col].unwrap().moving)
                {
                    is_complete = false;
                    break;
                }
            }

            if is_complete {
                rows_to_clear.push(row);
            }
        }

        if !rows_to_clear.is_empty() {
            let points = match rows_to_clear.len() {
                1 => 100,
                2 => 300,
                3 => 500,
                4 => 800,
                n => n * 100,
            };

            *score += points as u32;

            for &row_to_clear in rows_to_clear.iter().rev() {
                for col in 0..cols {
                    game_space[row_to_clear][col] = None;
                }

                for row in (1..=row_to_clear).rev() {
                    for col in 0..cols {
                        game_space[row][col] = game_space[row - 1][col].take();
                    }
                }
            }

            return rows_to_clear.len() as u32;
        }

        0
    }

}