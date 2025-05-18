use crate::bag::Bag;
use crate::shapes::{Shape};
use crate::types::Matrix;

pub fn move_right(game_space: &mut Matrix) {
    let rows = game_space.len();
    let cols = game_space[0].len();

    let mut can_move = true;

    for row in 0..rows {
        for col in 0..cols {
            if let Some(brick) = &game_space[row][col] {
                if brick.moving {
                    if col + 1 >= cols
                        || (game_space[row][col + 1].is_some()
                        && !game_space[row][col + 1].as_ref().unwrap().moving)
                    {
                        can_move = false;
                        break;
                    }
                }
            }
        }
        if !can_move {
            break;
        }
    }

    if can_move {
        for row in 0..rows {
            for col in (0..cols - 1).rev() {
                if let Some(brick) = &game_space[row][col] {
                    if brick.moving {
                        game_space[row][col + 1] =
                            game_space[row][col].take();
                    }
                }
            }
        }
    }
}

pub fn move_left(game_space: &mut Matrix) {
    let rows = game_space.len();
    let cols = game_space[0].len();

    let mut can_move = true;

    for row in 0..rows {
        for col in 0..cols {
            if let Some(brick) = &game_space[row][col] {
                if brick.moving {
                    if col == 0
                        || (game_space[row][col - 1].is_some()
                        && !game_space[row][col - 1].as_ref().unwrap().moving)
                    {
                        can_move = false;
                        break;
                    }
                }
            }
        }
        if !can_move {
            break;
        }
    }

    if can_move {
        for row in 0..rows {
            for col in 1..cols {
                if let Some(brick) = &game_space[row][col] {
                    if brick.moving {
                        game_space[row][col - 1] =
                            game_space[row][col].take();
                    }
                }
            }
        }
    }
}

pub fn move_bottom(game_space: &mut Matrix, bag: &mut Bag, new_next_item: &mut Shape) -> bool {
    let mut can_move = true;
    let rows = game_space.len();
    let cols = game_space[0].len();

    for row in (0..rows).rev() {
        for col in 0..cols {
            if let Some(brick) = &game_space[row][col] {
                if brick.moving {
                    if row + 1 >= rows
                        || (game_space[row + 1][col].is_some()
                        && !game_space[row + 1][col].as_ref().unwrap().moving)
                    {
                        can_move = false;
                        break;
                    }
                }
            }
        }
    }

    if can_move {
        for row in (0..rows - 1).rev() {
            for col in 0..cols {
                if let Some(brick) = &game_space[row][col] {
                    if brick.moving {
                        game_space[row + 1][col] =
                            game_space[row][col].take();
                    }
                }
            }
        }

        for col in 0..cols {
            game_space[0][col] = None;
        }

        false
    } else {
        let rows = game_space.len();
        let cols = game_space[0].len();

        for row in 0..rows {
            for col in 0..cols {
                if let Some(brick) = &mut game_space[row][col] {
                    if brick.moving {
                        brick.moving = false;
                    }
                }
            }
        }

        let (item, next_item) = bag.get_item();
        *new_next_item = next_item;

        let mut seen = false;
        let mut start_row = 0;
        let start_col = game_space[0].len() / 2 - 1;

        let (game_over, rows_to_render) = is_game_over(&game_space, new_next_item);

        if game_over {
            let rows_to_skip = item.matrix.len() - rows_to_render as usize;
            let mut row_index = 0;
            
            for (_, row) in item.matrix.iter().skip(rows_to_skip).enumerate() {
                seen = false;
                for (col_index, cell) in row.iter().enumerate() {
                    if let Some(brick) = cell {
                        seen = true;
                        let x = row_index;
                        let y = start_col + col_index;

                        let mut new_brick = brick.clone();

                        new_brick.moving = false;
                        game_space[x][y] = Some(new_brick);
                    }
                }

                if seen {
                    row_index += 1;
                }
            }
            
            return true;
        }

        for (_, row) in item.matrix.iter().enumerate() {
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

        false
    }
}

fn is_game_over(game_space: &Matrix, item: &Shape) -> (bool, u32) {
    let start_row = 0;
    let start_col = game_space[0].len() / 2 - item.matrix[0].len() / 2;
    let mut rows_to_render = 0;

    for (row_index, row) in item.matrix.iter().enumerate() {
        for (col_index, cell) in row.iter().enumerate() {
            if let Some(_) = cell {
                let game_row = start_row + row_index;
                let game_col = start_col + col_index;

                if game_row < game_space.len() && game_col < game_space[0].len() {
                    if game_space[game_row][game_col].is_some() {
                        return (true, rows_to_render);
                    }

                    rows_to_render += 1;
                }
            }
        }
    }

    (false, rows_to_render)
}
