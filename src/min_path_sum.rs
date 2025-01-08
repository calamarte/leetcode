#![allow(dead_code)]

use std::i32;

fn main() {
    let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];

    println!("Result -> {}", min_path_sum_up(grid));
}

/// # Top-down
/// Time complexity: $$O(n^2)$$
/// Space complexity: $$O(n^2)$$
fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    fn dfs(cords: (usize, usize), grid: &[Vec<i32>], cache: &mut [Vec<i32>]) -> i32 {
        let (y, x) = cords;

        if cache[y][x] != -1 {
            return cache[y][x];
        }

        if cords == (grid.len() - 1, grid[0].len() - 1) {
            return grid[y][x];
        }

        const DIRECTIONS: [(usize, usize); 2] = [(0, 1), (1, 0)];

        let mut min = i32::MAX;
        for (dy, dx) in DIRECTIONS {
            let ny = dy + y;
            let nx = dx + x;

            if ny >= grid.len() || nx >= grid[0].len() {
                continue;
            }

            min = dfs((ny, nx), grid, cache).min(min);
        }

        let result = grid[y][x] + min;
        cache[y][x] = result;

        result
    }

    dfs(
        (0, 0),
        &grid,
        &mut vec![vec![-1; grid[0].len()]; grid.len()],
    )
}

/// # Bottom-up
/// Time complexity: $$O(n^2)$$
/// Space complexity: $$O(n^2)$$
fn min_path_sum_up(grid: Vec<Vec<i32>>) -> i32 {
    let (h, w) = (grid.len(), grid[0].len());

    let mut cache = vec![vec![i32::MAX; grid[0].len() + 1]; grid.len() + 1];
    cache[h - 1][w] = 0;

    for y in (0..h).rev() {
        for x in (0..w).rev() {
            cache[y][x] = grid[y][x] + cache[y + 1][x].min(cache[y][x + 1]);
        }
    }

    cache[0][0]
}
