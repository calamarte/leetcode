/// # [62. Unique Paths](https://leetcode.com/problems/unique-paths/)
use std::collections::HashMap;

#[allow(unused_variables)]
fn main() {
    let (m, n) = (3, 7);
    let (m, n) = (3, 2);

    println!("Result top-down -> {}", unique_paths(m, n));
    println!("Result bottom-up -> {}", unique_paths_up(m, n));
    println!("Result bottom-up-mem -> {}", unique_paths_up_mem(m, n));
}

/// - Patterns: dp, top-down
/// - Time complexity: $$O(m \cdot n)$$
/// - Space complexity: $$O(m \cdot n)$$
fn unique_paths(m: i32, n: i32) -> i32 {
    fn dfs((y, x): (i32, i32), grid: &(i32, i32), cache: &mut HashMap<(i32, i32), i32>) -> i32 {
        if y < 1 || y > grid.0 || x < 1 || x > grid.1 {
            return 0;
        }

        if &(y, x) == grid {
            return 1;
        }

        if let Some(&v) = cache.get(&(y, x)) {
            return v;
        }

        let result = dfs((y + 1, x), grid, cache) + dfs((y, x + 1), grid, cache);
        cache.insert((y, x), result);

        result
    }

    dfs((1, 1), &(m, n), &mut HashMap::new())
}

/// - Patterns: dp, bottom-up
/// - Time complexity: $$O(m \cdot n)$$
/// - Space complexity: $$O(m \cdot n)$$
fn unique_paths_up(m: i32, n: i32) -> i32 {
    let (m, n) = (m as usize, n as usize);
    let mut dp = vec![vec![0; n + 1]; m + 1];

    dp[m][n - 1] = 1;

    for y in (0..m).rev() {
        for x in (0..n).rev() {
            dp[y][x] = dp[y + 1][x] + dp[y][x + 1];
        }
    }

    dp[0][0]
}

/// - Patterns: dp, bottom-up
/// - Time complexity: $$O(m \cdot n)$$
/// - Space complexity: $$O(n)$$
fn unique_paths_up_mem(m: i32, n: i32) -> i32 {
    let mut dp = vec![1; n as usize];

    for _ in 1..m {
        for x in (0..n as usize - 1).rev() {
            dp[x] += dp[x + 1];
        }
    }

    dp[0]
}
