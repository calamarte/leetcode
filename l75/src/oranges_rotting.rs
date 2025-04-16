use std::collections::{HashSet, VecDeque};

#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
    let grid = vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]];
    let grid = vec![vec![0, 2]];
    let grid = vec![vec![0]];
    let grid = vec![vec![1], vec![2]];

    println!("Result -> {}", oranges_rotting(grid));
}

/// - Patterns: DFS
/// - Time complexity: $$O(n \cdot m)$$
/// - Space complexity: $$O(n \cdot m)$$
fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
    const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

    let mut rotten = VecDeque::new();

    let mut oranges = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let cell = grid[y][x];

            if cell == 1 || cell == 2 {
                oranges += 1;
            }

            if cell == 2 {
                rotten.push_front((y, x, 0));
            }
        }
    }

    if oranges == 0 {
        return 0;
    }

    let mut visited = HashSet::new();
    while let Some((y, x, m)) = rotten.pop_back() {
        visited.insert((y, x));

        if visited.len() == oranges {
            return m;
        }

        grid[y][x] = 2;

        for (dy, dx) in DIRECTIONS {
            let ny = y as i32 + dy;
            let nx = x as i32 + dx;

            if ny >= 0
                && ny < grid.len() as i32
                && nx >= 0
                && nx < grid[0].len() as i32
                && grid[ny as usize][nx as usize] == 1
            {
                rotten.push_front((ny as usize, nx as usize, m + 1));
            }
        }
    }

    -1
}
