use std::collections::HashSet;

#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    let word = "ABCCED".to_string();
    let word = "SEE".to_string();
    let word = "ABCB".to_string();

    let board = vec![
        vec!['C', 'A', 'A'],
        vec!['A', 'A', 'A'],
        vec!['B', 'C', 'D'],
    ];
    let word = "AAB".to_string();

    println!("Result -> {}", exist(board, word));
}

fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    if word.is_empty() {
        return true;
    }

    fn backtrack(
        last_comb: (usize, usize),
        board: &[Vec<char>],
        index: usize,
        target: &[char],
        visited: &mut HashSet<(usize, usize)>,
    ) -> bool {
        if index == target.len() {
            return true;
        }

        let height = board.len();
        let width = board[0].len();

        let (y, x) = last_comb;
        let t = target[index];

        const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        for (dy, dx) in DIRECTIONS {
            let ny = y as isize + dy;
            let nx = x as isize + dx;

            if ny < 0
                || nx < 0
                || ny as usize >= height
                || nx as usize >= width
                || visited.contains(&(ny as usize, nx as usize))
                || board[ny as usize][nx as usize] != t
            {
                continue;
            }

            let cord = (ny as usize, nx as usize);
            visited.insert(cord);
            if backtrack(cord, board, index + 1, target, visited) {
                return true;
            }
            visited.remove(&cord);
        }

        false
    }

    let mut visited = HashSet::new();
    let word: Vec<_> = word.chars().collect();
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            let start = board[y][x];

            if word[0] != start {
                continue;
            }

            let cord = (y, x);
            visited.insert(cord);
            if backtrack(cord, &board, 1, &word, &mut visited) {
                return true;
            }
            visited.remove(&cord);
        }
    }

    false
}
