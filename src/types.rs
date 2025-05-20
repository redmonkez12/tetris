use crate::tetromino::{Brick, Tetromino};

pub type Matrix = Vec<Vec<Option<Brick>>>;
pub type Tetrominos = Vec<Tetromino>;
pub type TimeLocal = chrono::DateTime<chrono::Local>;