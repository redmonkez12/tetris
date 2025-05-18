use crate::shapes::{Brick, Shape};

pub type Matrix = Vec<Vec<Option<Brick>>>;
pub type Shapes = Vec<Shape>;
pub type TimeLocal = chrono::DateTime<chrono::Local>;