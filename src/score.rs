pub fn clear_rows(game_space: &mut Vec<Vec<Option<crate::shapes::Brick>>>, score: &mut u32) -> u32 {
    let rows = game_space.len();
    let cols = game_space[0].len();

    let mut rows_to_clear = Vec::new();

    for row in 0..rows {
        let mut is_complete = true;

        for col in 0..cols {
            if game_space[row][col].is_none() || (game_space[row][col].is_some() && game_space[row][col].unwrap().moving) {
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