use crate::tetromino::Tetromino;
use crate::types::Matrix;

pub fn move_right(game_space: &mut Matrix) {
    let rows = game_space.len();
    let cols = game_space[0].len();

    let mut moving_bricks = Vec::new();

    for col in 0..cols {
        for row in 0..rows {
            if let Some(brick) = &game_space[row][col] {
                if brick.moving {
                    moving_bricks.push((row, col));
                }
            }
        }
    }

    if moving_bricks.is_empty() {
        return;
    }

    let mut can_move = true;

    for &(row, col) in moving_bricks.iter() {
        if col + 1 >= cols
            || (game_space[row][col + 1].is_some()
            && !game_space[row][col + 1].as_ref().unwrap().moving)
        {
            can_move = false;
            break;
        }
    }

    if can_move {
        for &(row, col) in moving_bricks.iter().rev() {
            game_space[row][col + 1] = game_space[row][col].take();
        }
    }
}

pub fn move_left(game_space: &mut Matrix) {
    let rows = game_space.len();
    let cols = game_space[0].len();

    let mut moving_bricks = Vec::new();

    for col in 0..cols {
        for row in 0..rows {
            if let Some(brick) = &game_space[row][col] {
                if brick.moving {
                    moving_bricks.push((row, col));
                }
            }
        }
    }

    if moving_bricks.is_empty() {
        return;
    }

    let mut can_move = true;

    for &(row, col) in moving_bricks.iter() {
        if col == 0
            || (game_space[row][col - 1].is_some()
            && !game_space[row][col - 1].as_ref().unwrap().moving)
        {
            can_move = false;
            break;
        }
    }

    if can_move {
        for (row, col) in moving_bricks {
            game_space[row][col - 1] = game_space[row][col].take();
        }
    }
}

pub fn move_bottom(game_space: &mut Matrix) -> bool {
    let rows = game_space.len();
    let cols = game_space[0].len();
    let mut can_move = true;

    for row in 0..rows {
        for col in 0..cols {
            if let Some(brick) = &game_space[row][col] {
                if brick.moving {
                    if row + 1 >= rows {
                        can_move = false;
                        break;
                    }

                    if let Some(other_brick) = &game_space[row + 1][col] {
                        if !other_brick.moving {
                            can_move = false;
                            break;
                        }
                    }
                }
            }
        }

        if !can_move {
            break;
        }
    }

    if !can_move {
        for row in 0..rows {
            for col in 0..cols {
                if let Some(brick) = &mut game_space[row][col] {
                    if brick.moving {
                        brick.moving = false;
                    }
                }
            }
        }

        return false;
    }

    for row in (0..rows - 1).rev() {
        for col in 0..cols {
            if let Some(brick) = &game_space[row][col] {
                if brick.moving {
                    let brick = game_space[row][col].take();
                    game_space[row + 1][col] = brick;
                }
            }
        }
    }

    true
}

pub fn draw_game_over_brick(
    game_space: &mut Matrix,
    start_col: usize,
    rows_to_render: u32,
    item: &Tetromino,
) {
    let rows_to_skip = item.matrix.len() - rows_to_render as usize;
    let mut row_index = 0;

    for (_, row) in item.matrix.iter().skip(rows_to_skip).enumerate() {
        let has_brick = row.iter().any(|cell| cell.is_some());

        if !has_brick {
            continue;
        }

        for (col_index, cell) in row.iter().enumerate() {
            if let Some(brick) = cell {
                let x = row_index;
                let y = start_col + col_index;

                let mut new_brick = brick.clone();

                new_brick.moving = false;
                game_space[x][y] = Some(new_brick);
            }
        }

        row_index += 1;
    }
}

pub fn is_game_over(game_space: &Matrix, item: &Tetromino) -> (bool, u32) {
    let start_col = game_space[0].len() / 2 - 1;
    let mut rows_to_render = 0;
    let mut seen = true;

    for row in item.matrix.iter() {
        seen = false;

        for (col_index, cell) in row.iter().enumerate() {
            if let Some(_) = cell {
                seen = true;
                let game_row = rows_to_render;
                let game_col = start_col + col_index;

                if game_space[game_row][game_col].is_some() {
                    return (true, rows_to_render as u32);
                }
            }
        }

        if seen {
            rows_to_render += 1;
        }
    }

    (false, rows_to_render as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tetromino::Brick;
    use iced::Color;

    #[test]
    fn test_game_over_i() {
        let moving_brick = Some(Brick::new(Color::from_rgb(1.0, 1.0, 0.0), false));
        let shape = Tetromino::create_i();

        let game_space: Matrix = vec![
            vec![
                None,
                None,
                None,
                moving_brick,
                moving_brick,
                moving_brick,
                moving_brick,
                None,
                None,
                None,
            ],
            vec![
                None,
                None,
                None,
                moving_brick,
                moving_brick,
                moving_brick,
                moving_brick,
                None,
                None,
                None,
            ],
            vec![None; 10],
        ];

        let (game_over, rows_to_render) = is_game_over(&game_space, &shape);

        assert_eq!(game_over, true);
        assert_eq!(rows_to_render, 0);
    }

    #[test]
    fn test_game_over_o() {
        let moving_brick = Some(Brick::new(Color::from_rgb(1.0, 1.0, 0.0), false));
        let shape = Tetromino::create_o();

        let game_space: Matrix = vec![
            vec![None; 10],
            vec![
                None,
                None,
                None,
                moving_brick,
                moving_brick,
                moving_brick,
                moving_brick,
                None,
                None,
                None,
            ],
            vec![
                None,
                None,
                None,
                moving_brick,
                moving_brick,
                moving_brick,
                moving_brick,
                None,
                None,
                None,
            ],
        ];

        let (game_over, rows_to_render) = is_game_over(&game_space, &shape);

        assert_eq!(game_over, true);
        assert_eq!(rows_to_render, 1);
    }

    #[test]
    fn test_game_over_l() {
        let moving_brick = Some(Brick::new(Color::from_rgb(1.0, 1.0, 0.0), false));
        let shape = Tetromino::create_l();

        let game_space: Matrix = vec![
            vec![None; 10],
            vec![None; 10],
            vec![
                None,
                moving_brick,
                moving_brick,
                moving_brick,
                moving_brick,
                moving_brick,
                moving_brick,
                None,
                None,
                None,
            ],
            vec![
                None,
                moving_brick,
                moving_brick,
                moving_brick,
                moving_brick,
                moving_brick,
                moving_brick,
                None,
                None,
                None,
            ],
            vec![None; 10],
        ];

        let (game_over, rows_to_render) = is_game_over(&game_space, &shape);

        assert_eq!(game_over, true);
        assert_eq!(rows_to_render, 2);
    }

    #[test]
    fn test_game_over_z() {
        let moving_brick = Some(Brick::new(Color::from_rgb(1.0, 1.0, 0.0), false));
        let shape = Tetromino::create_z();

        let game_space: Matrix = vec![
            vec![None; 10],
            vec![None; 10],
            vec![
                None,
                None,
                moving_brick,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            vec![None; 10],
            vec![None; 10],
        ];

        let (game_over, rows_to_render) = is_game_over(&game_space, &shape);

        assert_eq!(game_over, false);
        assert_eq!(rows_to_render, 2);
    }

    #[test]
    fn test_empty_game_space() {
        let mut game_space: Matrix = vec![vec![None; 10], vec![None; 10], vec![None; 10]];

        move_left(&mut game_space);

        let expected_game_space: Matrix = vec![vec![None; 10], vec![None; 10], vec![None; 10]];

        assert_eq!(game_space, expected_game_space);
    }

    #[test]
    fn test_left_only_non_moving_bricks() {
        let non_moving_brick = Some(Brick::new(Color::from_rgb(1.0, 1.0, 0.0), false));

        let mut game_space: Matrix = vec![
            vec![None; 10],
            vec![
                None,
                None,
                non_moving_brick,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            vec![
                None,
                non_moving_brick,
                non_moving_brick,
                non_moving_brick,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            vec![None; 10],
        ];

        move_left(&mut game_space);

        let expected_game_space: Matrix = vec![
            vec![None; 10],
            vec![
                None,
                None,
                non_moving_brick,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            vec![
                None,
                non_moving_brick,
                non_moving_brick,
                non_moving_brick,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            vec![None; 10],
        ];

        assert_eq!(game_space, expected_game_space);
    }

    #[test]
    fn test_left_moving_bricks() {
        let brick = Some(Brick::new(Color::from_rgb(1.0, 1.0, 0.0), true));

        let mut game_space: Matrix = vec![
            vec![None; 10],
            vec![None, None, brick, None, None, None, None, None, None, None],
            vec![
                None, None, brick, brick, brick, None, None, None, None, None,
            ],
            vec![None; 10],
        ];

        move_left(&mut game_space);

        let expected_game_space: Matrix = vec![
            vec![None; 10],
            vec![None, brick, None, None, None, None, None, None, None, None],
            vec![
                None, brick, brick, brick, None, None, None, None, None, None,
            ],
            vec![None; 10],
        ];

        assert_eq!(game_space, expected_game_space);
    }

    #[test]
    fn test_left_collision_case() {
        let brick = Some(Brick::new(Color::from_rgb(1.0, 1.0, 0.0), true));
        let non_moving_brick = Some(Brick::new(Color::from_rgb(1.0, 1.0, 0.0), false));

        let mut game_space: Matrix = vec![
            vec![None; 10],
            vec![
                None,
                non_moving_brick,
                brick,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            vec![None, None, brick, brick, None, None, None, None, None, None],
            vec![None; 10],
        ];

        move_left(&mut game_space);

        let expected_game_space: Matrix = vec![
            vec![None; 10],
            vec![
                None,
                non_moving_brick,
                brick,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            vec![None, None, brick, brick, None, None, None, None, None, None],
            vec![None; 10],
        ];

        assert_eq!(game_space, expected_game_space);
    }

    #[test]
    fn test_right_moving_bricks() {
        let brick = Some(Brick::new(Color::from_rgb(1.0, 1.0, 0.0), true));
        let non_moving_brick = Some(Brick::new(Color::from_rgb(1.0, 1.0, 0.0), false));

        let mut game_space: Matrix = vec![
            vec![None; 10],
            vec![None, brick, None, None, None, None, None, None, None, None],
            vec![
                None, brick, brick, brick, None, None, None, None, None, None,
            ],
            vec![None; 10],
            vec![
                non_moving_brick,
                non_moving_brick,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            vec![
                non_moving_brick,
                non_moving_brick,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
        ];

        move_right(&mut game_space);

        let expected_game_space: Matrix = vec![
            vec![None; 10],
            vec![None, None, brick, None, None, None, None, None, None, None],
            vec![
                None, None, brick, brick, brick, None, None, None, None, None,
            ],
            vec![None; 10],
            vec![
                non_moving_brick,
                non_moving_brick,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            vec![
                non_moving_brick,
                non_moving_brick,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
        ];

        assert_eq!(game_space, expected_game_space);

        move_right(&mut game_space);

        let expected_game_space: Matrix = vec![
            vec![None; 10],
            vec![None, None, None, brick, None, None, None, None, None, None],
            vec![
                None, None, None, brick, brick, brick, None, None, None, None,
            ],
            vec![None; 10],
            vec![
                non_moving_brick,
                non_moving_brick,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            vec![
                non_moving_brick,
                non_moving_brick,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
        ];

        assert_eq!(game_space, expected_game_space);
    }

    #[test]
    fn test_right_collision_case() {
        let brick = Some(Brick::new(Color::from_rgb(1.0, 1.0, 0.0), true));
        let non_moving_brick = Some(Brick::new(Color::from_rgb(1.0, 1.0, 0.0), false));

        let mut game_space: Matrix = vec![
            vec![None; 10],
            vec![
                None,
                None,
                brick,
                non_moving_brick,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            vec![None, None, brick, brick, None, None, None, None, None, None],
            vec![None; 10],
        ];

        move_right(&mut game_space);

        let expected_game_space: Matrix = vec![
            vec![None; 10],
            vec![
                None,
                None,
                brick,
                non_moving_brick,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            vec![None, None, brick, brick, None, None, None, None, None, None],
            vec![None; 10],
        ];

        assert_eq!(game_space, expected_game_space);
    }

    #[test]
    fn test_bottom_collision_case() {
        let brick = Some(Brick::new(Color::from_rgb(1.0, 1.0, 0.0), true));
        let non_moving_brick = Some(Brick::new(Color::from_rgb(1.0, 1.0, 0.0), false));

        let mut game_space: Matrix = vec![
            vec![None; 10],
            vec![
                None,
                None,
                brick,
                non_moving_brick,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            vec![None, None, brick, brick, None, None, None, None, None, None],
            vec![
                None,
                non_moving_brick,
                non_moving_brick,
                non_moving_brick,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            vec![None; 10],
        ];

        move_right(&mut game_space);

        let expected_game_space: Matrix = vec![
            vec![None; 10],
            vec![
                None,
                None,
                brick,
                non_moving_brick,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            vec![None, None, brick, brick, None, None, None, None, None, None],
            vec![
                None,
                non_moving_brick,
                non_moving_brick,
                non_moving_brick,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            vec![None; 10],
        ];

        assert_eq!(game_space, expected_game_space);
    }

    #[test]
    fn test_bottom_moving_bricks() {
        let brick = Some(Brick::new(Color::from_rgb(1.0, 1.0, 0.0), true));

        let mut game_space: Matrix = vec![
            vec![None; 10],
            vec![None, brick, None, None, None, None, None, None, None, None],
            vec![
                None, brick, brick, brick, None, None, None, None, None, None,
            ],
            vec![None; 10],
        ];

        move_bottom(&mut game_space);

        let expected_game_space: Matrix = vec![
            vec![None; 10],
            vec![None; 10],
            vec![None, brick, None, None, None, None, None, None, None, None],
            vec![
                None, brick, brick, brick, None, None, None, None, None, None,
            ],
        ];

        assert_eq!(game_space, expected_game_space);
    }

    #[test]
    fn test_draw_game_over_brick_1() {
        let moving_brick = Some(Brick::new(Color::from_rgb(1.0, 1.0, 0.0), false));
        let shape = Tetromino::create_i();

        let mut game_space: Matrix = vec![vec![
            None,
            moving_brick,
            moving_brick,
            moving_brick,
            moving_brick,
            moving_brick,
            moving_brick,
            None,
            None,
            None,
        ]];

        draw_game_over_brick(&mut game_space, 5, 0, &shape);

        let expected_game_space: Matrix = vec![vec![
            None,
            moving_brick,
            moving_brick,
            moving_brick,
            moving_brick,
            moving_brick,
            moving_brick,
            None,
            None,
            None,
        ]];

        assert_eq!(game_space, expected_game_space);
    }
}
