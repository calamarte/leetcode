fn main() {
    #[rustfmt::skip]
    let mut board = vec![
        vec![0, 1, 0],
        vec![0, 0, 1],
        vec![1, 1, 1],
        vec![0, 0, 0]
    ];

    game_of_life(&mut board);
    println!("Result -> {board:?}");
}

#[allow(clippy::ptr_arg)]
fn game_of_life(board: &mut Vec<Vec<i32>>) {
    let mut to_toggle: Vec<(usize, usize)> = Vec::new();

    for y in 0..board.len() {
        for x in 0..board[0].len() {
            let cords = (y, x);
            if need_toggle(&cords, board) {
                to_toggle.push(cords);
            }
        }
    }

    println!("{to_toggle:?}");

    for (y, x) in to_toggle {
        let cell = &mut board[y][x];
        *cell = if *cell == 1 { 0 } else { 1 };
    }
}

fn need_toggle(cords: &(usize, usize), board: &[Vec<i32>]) -> bool {
    let mut alive = 0;
    for y_offset in -1..=1 {
        for x_offset in -1..=1 {
            if y_offset == 0 && x_offset == 0 {
                continue;
            }

            let (y, x) = (cords.0 as i32 + y_offset, cords.1 as i32 + x_offset);

            if y >= 0
                && y < board.len() as i32
                && x >= 0
                && x < board[0].len() as i32
                && board[y as usize][x as usize] == 1
            {
                alive += 1;
            }
        }
    }

    match board[cords.0][cords.1] {
        0 => alive == 3,
        1 => !(2..=3).contains(&alive),
        _ => false,
    }
}
