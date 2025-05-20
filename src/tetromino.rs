use crate::types::Matrix;
use iced::Color;
use crate::colors::{COLOR_I, COLOR_J, COLOR_L, COLOR_O, COLOR_S, COLOR_T, COLOR_Z};

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
pub struct Tetromino {
    pub matrix: Matrix,
}

impl Tetromino {
    pub fn new(matrix: Vec<Vec<Option<Brick>>>) -> Self {
        Self { matrix }
    }

    pub fn create_o() -> Self {
        let b = Some(Brick::new(COLOR_O.into(), false));
        Self::new(vec![vec![b, b], vec![b, b]])
    }

    pub fn create_i() -> Self {
        let b = Some(Brick::new(COLOR_I.into(), false));
        let e = None;

        Self::new(vec![
            vec![e, b, e, e],
            vec![e, b, e, e],
            vec![e, b, e, e],
            vec![e, b, e, e],
        ])
    }

    pub fn create_s() -> Self {
        let b = Some(Brick::new(COLOR_S.into(), false));
        let e = None;

        Self::new(vec![vec![e, e, e], vec![e, b, b], vec![b, b, e]])
    }

    pub fn create_z() -> Self {
        let b = Some(Brick::new(COLOR_Z.into(), false));
        let e = None;

        Self::new(vec![vec![e, e, e], vec![b, b, e], vec![e, b, b]])
    }

    pub fn create_t() -> Self {
        let b = Some(Brick::new(COLOR_T.into(), false));
        let e = None;

        Self::new(vec![vec![e, e, e], vec![b, b, b], vec![e, b, e]])
    }

    pub fn create_l() -> Self {
        let b = Some(Brick::new(COLOR_L.into(), false));
        let e = None;

        Self::new(vec![vec![e, b, e], vec![e, b, e], vec![e, b, b]])
    }

    pub fn create_j() -> Self {
        let b = Some(Brick::new(COLOR_J.into(), false));
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

                    new_brick.moving = true;
                    game_space[x][y] = Some(new_brick);
                }
            }

            if seen {
                start_row += 1
            }
        }
    }
}
