#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let cost = vec![10, 15, 20];
    let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];

    println!("Result -> {}", min_cost_climbing_stairs(cost.clone()));
    println!(
        "Result bottom-up -> {}",
        min_cost_climbing_stairs_pro(cost.clone())
    );
    println!(
        "Result bottom-up-mem -> {}",
        min_cost_climbing_stairs_pro_mem(cost.clone())
    );
}

/// - Patterns: dp, top-down
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    fn dfs(index: usize, cost: &[i32], cache: &mut [i32]) -> i32 {
        if index >= cost.len() {
            return 0;
        }

        if cache[index] > -1 {
            return cache[index];
        }

        cache[index] = dfs(index + 1, cost, cache).min(dfs(index + 2, cost, cache)) + cost[index];
        cache[index]
    }

    let cache = &mut vec![-1; cost.len()];
    dfs(0, &cost, cache).min(dfs(1, &cost, cache))
}

/// - Patterns: dp, bottom-up
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
fn min_cost_climbing_stairs_pro(cost: Vec<i32>) -> i32 {
    let mut dp = vec![0; cost.len() + 2];

    for i in (0..cost.len()).rev() {
        dp[i] = dp[i + 1].min(dp[i + 2]) + cost[i];
    }

    dp[0].min(dp[1])
}

/// - Patterns: dp, bottom-up
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(1)$$
fn min_cost_climbing_stairs_pro_mem(cost: Vec<i32>) -> i32 {
    let (mut a, mut b) = (0, 0);

    for i in (0..cost.len()).rev() {
        (a, b) = (a.min(b) + cost[i], a);
    }

    a.min(b)
}
