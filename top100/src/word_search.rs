///
/// # [79. Word Search](https://leetcode.com/problems/word-search/)
///
use std::collections::HashSet;

#[allow(unused_variables)]
fn main() {
    let (board, word) = (
        vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ],
        String::from("ABCCED"),
    );

    let (board, word) = (
        vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ],
        String::from("SEE"),
    );

    let (board, word) = (
        vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ],
        String::from("ABCB"),
    );

    let (board, word) = (vec![vec!['a']], String::from("a"));
    // let (board, word) = (vec![vec!['a', 'b']], String::from("ba"));

    println!("Result -> {}", exist(board, word));
}

/// - Patterns: Backtracking
/// - Time complexity: $$O(n * m * 4^L)$$
/// - Space complexity: $$O(m * n)$$
fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    fn dfs(
        (y, x): (usize, usize),
        index: usize,
        grid: &[Vec<char>],
        target: &[char],
        visited: &mut HashSet<(usize, usize)>,
    ) -> bool {
        if index == target.len() - 1 {
            return !visited.contains(&(y, x)) && grid[y][x] == target[index];
        }

        if visited.contains(&(y, x)) || grid[y][x] != target[index] {
            return false;
        }

        visited.insert((y, x));

        for (dy, dx) in DIRECTIONS {
            let (ny, nx) = (y as isize + dy, x as isize + dx);

            if ny < 0
                || ny >= grid.len() as isize
                || nx < 0
                || nx >= grid[ny as usize].len() as isize
            {
                continue;
            }

            let coords = (ny as usize, nx as usize);
            if dfs(coords, index + 1, grid, target, visited) {
                return true;
            }
        }

        visited.remove(&(y, x));

        false
    }

    let target: Vec<_> = word.chars().collect();
    let visited = &mut HashSet::new();
    for y in 0..board.len() {
        for x in 0..board[y].len() {
            if dfs((y, x), 0, &board, &target, visited) {
                return true;
            }
        }
    }

    false
}
