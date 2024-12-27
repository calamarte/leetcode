use std::collections::HashSet;

#[allow(unused_variables, unused_mut, clippy::useless_vec)]
fn main() {
    let mut board = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];

    let mut board = vec![
        vec!['O', 'O', 'O'],
        vec!['O', 'X', 'O'],
        vec!['O', 'O', 'O'],
    ];

    let mut board = vec![
        vec!['O', 'X', 'X', 'O', 'X'],
        vec!['X', 'O', 'O', 'X', 'O'],
        vec!['X', 'O', 'X', 'O', 'X'],
        vec!['O', 'X', 'O', 'O', 'O'],
        vec!['X', 'X', 'O', 'X', 'O'],
    ];

    solve(&mut board);
    println!("Result -> ");
    board_print(&board);
}

/// Time complexity: $$O(m /cdot n)$$
/// Space complexity: $$O(m /cdot n)$$
#[allow(clippy::ptr_arg)]
fn solve(board: &mut Vec<Vec<char>>) {
    /// Interactive DFS
    fn surrround(board: &[Vec<char>], start: (usize, usize), free: &mut HashSet<(usize, usize)>) {
        const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

        let (h, w) = (board.len(), board[0].len());

        let mut stack = vec![start];
        while let Some((y, x)) = stack.pop() {
            if free.contains(&(y, x)) || board[y][x] == 'X' {
                continue;
            }

            free.insert((y, x));

            for (dy, dx) in DIRECTIONS {
                let ny = y as isize + dy;
                let nx = x as isize + dx;

                if ny >= 0 && nx >= 0 && (ny as usize) < h && (nx as usize) < w {
                    stack.push((ny as usize, nx as usize));
                }
            }
        }
    }

    let mut free: HashSet<(usize, usize)> = HashSet::new();

    // left col
    for y in 1..board.len() - 1 {
        surrround(board, (y, 0), &mut free);
    }

    // right col
    for y in 1..board.len() - 1 {
        surrround(board, (y, board[0].len() - 1), &mut free);
    }

    // top
    for x in 1..board[0].len() - 1 {
        surrround(board, (0, x), &mut free);
    }

    // bottom
    for x in 1..board[0].len() - 1 {
        surrround(board, (board.len() - 1, x), &mut free);
    }

    for y in 1..board.len() - 1 {
        for x in 1..board[0].len() - 1 {
            if board[y][x] == 'X' || free.contains(&(y, x)) {
                continue;
            }

            board[y][x] = 'X';
        }
    }
}

fn board_print(board: &[Vec<char>]) {
    for v in board {
        println!("{v:?}");
    }
}
