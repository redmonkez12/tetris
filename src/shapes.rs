use crate::types::Matrix;
use iced::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Brick {
    pub color: Color,
    pub moving: bool,
}

impl Brick {
    pub fn new(color: Color, moving: bool) -> Self {
        Brick { color, moving }
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Shape {
    pub matrix: Matrix,
}

impl Shape {
    pub fn new(matrix: Vec<Vec<Option<Brick>>>) -> Self {
        Self { matrix }
    }

    pub fn create_o() -> Self {
        let color = Color::from_rgb(1.0, 1.0, 0.0);
        let b = Some(Brick::new(color, false));
        Self::new(vec![vec![b, b], vec![b, b]])
    }

    pub fn create_i() -> Self {
        let color = Color::from_rgb(0.0, 1.0, 1.0);
        let b = Some(Brick::new(color, false));
        let e = None;

        Self::new(vec![
            vec![e, b, e, e],
            vec![e, b, e, e],
            vec![e, b, e, e],
            vec![e, b, e, e],
        ])
    }

    pub fn create_s() -> Self {
        let color = Color::from_rgb(0.0, 1.0, 0.0);
        let b = Some(Brick::new(color, false));
        let e = None;

        Self::new(vec![vec![e, e, e], vec![e, b, b], vec![b, b, e]])
    }

    pub fn create_z() -> Self {
        let color = Color::from_rgb(1.0, 0.0, 0.0);
        let b = Some(Brick::new(color, false));
        let e = None;

        Self::new(vec![vec![e, e, e], vec![b, b, e], vec![e, b, b]])
    }

    pub fn create_t() -> Self {
        let color = Color::from_rgb(0.5, 0.0, 0.5);
        let b = Some(Brick::new(color, false));
        let e = None;

        Self::new(vec![vec![e, e, e], vec![b, b, b], vec![e, b, e]])
    }

    pub fn create_l() -> Self {
        let color = Color::from_rgb(1.0, 0.5, 0.0);
        let b = Some(Brick::new(color, false));
        let e = None;

        Self::new(vec![vec![e, b, e], vec![e, b, e], vec![e, b, b]])
    }

    pub fn create_j() -> Self {
        let color = Color::from_rgb(0.0, 0.0, 1.0);
        let b = Some(Brick::new(color, false));
        let e = None;

        Self::new(vec![vec![e, b, e], vec![e, b, e], vec![b, b, e]])
    }

    pub fn set_default_position(&mut self, game_space: &mut Matrix) {
        let mut start_row = 0;
        let start_col = game_space[0].len() / 2 - 1;
        let mut seen = true;

        for (_, row) in self.matrix.iter().enumerate() {
            seen = false;

            for (col_index, cell) in row.iter().enumerate() {
                if let Some(brick) = cell {
                    seen = true;
                    let x = start_row;
                    let y = start_col + col_index;

                    let mut new_brick = brick.clone();

                    if x < game_space.len() && y < game_space[0].len() {
                        new_brick.moving = true;
                        game_space[x][y] = Some(new_brick);
                    }
                }
            }

            if seen {
                start_row += 1
            }
        }
    }
}
