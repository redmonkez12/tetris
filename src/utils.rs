use crate::shapes::{Shape};

pub fn rotate_clockwise(shape: &mut Shape) {
    let n = shape.matrix.len();
    let mut rotated = vec![vec![None; n]; n];

    for i in 0..n {
        for j in 0..n {
            rotated[j][n - 1 - i] = shape.matrix[i][j].clone();
        }
    }

    shape.matrix = rotated;
}

pub fn rotate_counterclockwise(shape: &mut Shape) {
    let n = shape.matrix.len();
    let mut rotated = vec![vec![None; n]; n];

    for i in 0..n {
        for j in 0..n {
            rotated[n - 1 - j][i] = shape.matrix[i][j].clone();
        }
    }

    shape.matrix = rotated;
}

