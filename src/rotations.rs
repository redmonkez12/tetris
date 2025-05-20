use crate::tetromino::{Brick};
use crate::types::Matrix;

fn find_size(rows: usize, cols: usize, game_space: &Matrix) -> Option<(usize, usize, usize)> {
    let mut min_row = rows;
    let mut min_col = cols;
    let mut max_row = 0;
    let mut max_col = 0;

    for row in 0..rows {
        for col in 0..cols {
            if let Some(brick) = &game_space[row][col] {
                if brick.moving {
                    min_row = min_row.min(row);
                    min_col = min_col.min(col);
                    max_row = max_row.max(row);
                    max_col = max_col.max(col);
                }
            }
        }
    }

    if min_row > max_row || min_col > max_col {
        return None;
    }

    let piece_height = max_row - min_row + 1;
    let piece_width = max_col - min_col + 1;
    let size = piece_height.max(piece_width);

    Some((size, min_row, min_col))
}

fn rotate_clockwise_matrix(
    size: usize,
    rows: usize,
    cols: usize,
    min_row: usize,
    min_col: usize,
    game_space: &Matrix,
    matrix_shape: &mut Matrix,
) -> Matrix {
    for row in 0..size {
        for col in 0..size {
            let game_row = min_row + row;
            let game_col = min_col + col;

            if game_row < rows && game_col < cols {
                if let Some(brick) = &game_space[game_row][game_col] {
                    if brick.moving {
                        matrix_shape[row][col] = game_space[game_row][game_col].clone();
                    }
                }
            }
        }
    }

    let mut rotated = vec![vec![None; size]; size];
    for i in 0..size {
        for j in 0..size {
            rotated[j][size - 1 - i] = matrix_shape[i][j].clone();
        }
    }

    rotated
}

fn can_rotate_clockwise(
    size: usize,
    rows: usize,
    cols: usize,
    min_row: usize,
    min_col: usize,
    game_space: &Matrix,
    rotated: &Matrix,
) -> bool {
    let mut can_rotate = true;

    for row in 0..size {
        for col in 0..size {
            if rotated[row][col].is_some() {
                let game_row = min_row + row;
                let game_col = min_col + col;

                if game_row >= rows || game_col >= cols || game_row < 0 || game_col < 0 {
                    can_rotate = false;
                    break;
                }

                if let Some(brick) = &game_space[game_row][game_col] {
                    if !brick.moving {
                        can_rotate = false;
                        break;
                    }
                }
            }
        }
        if !can_rotate {
            break;
        }
    }

    can_rotate
}

pub fn rotate_clockwise(game_space: &mut Vec<Vec<Option<Brick>>>) {
    let rows = game_space.len();
    let cols = game_space[0].len();

    if let Some((size, min_row, min_col)) = find_size(rows, cols, game_space) {
        let mut matrix_shape = vec![vec![None; size]; size];

        let rotated = rotate_clockwise_matrix(
            size,
            rows,
            cols,
            min_row,
            min_col,
            game_space,
            &mut matrix_shape,
        );

        let can_rotate =
            can_rotate_clockwise(size, rows, cols, min_row, min_col, game_space, &rotated);

        if can_rotate {
            for row in 0..rows {
                for col in 0..cols {
                    if let Some(brick) = &game_space[row][col] {
                        if brick.moving {
                            game_space[row][col] = None;
                        }
                    }
                }
            }

            for row in 0..size {
                for col in 0..size {
                    if let Some(_) = &rotated[row][col] {
                        let game_row = min_row + row;
                        let game_col = min_col + col;

                        if game_row < rows && game_col < cols {
                            game_space[game_row][game_col] = rotated[row][col].clone();
                        }
                    }
                }
            }
        }
    }
}

fn rotate_counterclockwise_matrix(
    size: usize,
    rows: usize,
    cols: usize,
    min_row: usize,
    min_col: usize,
    game_space: &Matrix,
    matrix_shape: &mut Matrix,
) -> Matrix {
    for row in 0..size {
        for col in 0..size {
            let game_row = min_row + row;
            let game_col = min_col + col;

            if game_row < rows && game_col < cols {
                if let Some(brick) = &game_space[game_row][game_col] {
                    if brick.moving {
                        matrix_shape[row][col] = game_space[game_row][game_col].clone();
                    }
                }
            }
        }
    }

    let mut rotated = vec![vec![None; size]; size];
    for i in 0..size {
        for j in 0..size {
            rotated[size - 1 - j][i] = matrix_shape[i][j].clone();
        }
    }

    rotated
}

fn can_rotate_counterclockwise(
    size: usize,
    rows: usize,
    cols: usize,
    min_row: usize,
    min_col: usize,
    game_space: &Matrix,
    rotated: &Matrix,
) -> bool {
    let mut can_rotate = true;

    for row in 0..size {
        for col in 0..size {
            if rotated[row][col].is_some() {
                let game_row = min_row + row;
                let game_col = min_col + col;

                if game_row >= rows || game_col >= cols || game_row < 0 || game_col < 0 {
                    can_rotate = false;
                    break;
                }

                if let Some(brick) = &game_space[game_row][game_col] {
                    if !brick.moving {
                        can_rotate = false;
                        break;
                    }
                }
            }
        }
        if !can_rotate {
            break;
        }
    }

    can_rotate
}

pub fn rotate_counterclockwise(game_space: &mut Vec<Vec<Option<Brick>>>) {
    let rows = game_space.len();
    let cols = game_space[0].len();

    if let Some((size, min_row, min_col)) = find_size(rows, cols, game_space) {
        let mut matrix_shape = vec![vec![None; size]; size];

        let rotated = rotate_counterclockwise_matrix(
            size,
            rows,
            cols,
            min_row,
            min_col,
            game_space,
            &mut matrix_shape,
        );

        let can_rotate =
            can_rotate_counterclockwise(size, rows, cols, min_row, min_col, game_space, &rotated);

        if can_rotate {
            for row in 0..rows {
                for col in 0..cols {
                    if let Some(brick) = &game_space[row][col] {
                        if brick.moving {
                            game_space[row][col] = None;
                        }
                    }
                }
            }

            for row in 0..size {
                for col in 0..size {
                    if let Some(brick) = &rotated[row][col] {
                        let game_row = min_row + row;
                        let game_col = min_col + col;

                        if game_row < rows && game_col < cols {
                            game_space[game_row][game_col] = rotated[row][col].clone();
                        }
                    }
                }
            }
        }
    }
}
