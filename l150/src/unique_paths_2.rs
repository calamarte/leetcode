#![allow(dead_code)]

#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let obstacle_grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
    let obstacle_grid = vec![vec![0, 1], vec![0, 0]];
    let obstacle_grid = vec![vec![0, 0]];
    // let obstacle_grid = vec![vec![0, 0], vec![0, 1]];

    println!(
        "Result -> {}",
        unique_paths_with_obstacles_up(obstacle_grid)
    );
}

/// # Top-down
/// Time complexity: $$O(n \cdot m)$$
/// Space complexity: $$O(n \cdot m)$$
fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    fn dfs(cords: (usize, usize), grid: &[Vec<i32>], cache: &mut [Vec<i32>]) -> i32 {
        let (y, x) = cords;

        if cache[y][x] != -1 {
            return cache[y][x];
        }

        if cords == (grid.len() - 1, grid[0].len() - 1) {
            return 1;
        }

        const DIRECTIONS: [(usize, usize); 2] = [(0, 1), (1, 0)];

        let mut total = 0;
        for (dy, dx) in DIRECTIONS {
            let ny = dy + y;
            let nx = dx + x;

            if ny >= grid.len() || nx >= grid[0].len() || grid[y][x] == 1 {
                continue;
            }

            total += dfs((ny, nx), grid, cache);
        }

        cache[y][x] = total;

        total
    }

    if obstacle_grid[0][0] == 1
        || obstacle_grid[obstacle_grid.len() - 1][obstacle_grid[0].len() - 1] == 1
    {
        return 0;
    }

    dfs(
        (0, 0),
        &obstacle_grid,
        &mut vec![vec![-1; obstacle_grid[0].len()]; obstacle_grid.len()],
    )
}

/// # Bottom-up
/// Time complexity: $$O(n \cdot m)$$
/// Space complexity: $$O(n)$$
fn unique_paths_with_obstacles_up(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let (h, w) = (obstacle_grid.len(), obstacle_grid[0].len());

    let mut cache = vec![0; w];
    cache[w - 1] = 1;

    for y in (0..h).rev() {
        for x in (0..w).rev() {
            if obstacle_grid[y][x] == 1 {
                cache[x] = 0;
            } else if x + 1 < w {
                cache[x] += cache[x + 1];
            }
        }
    }

    cache[0]
}
