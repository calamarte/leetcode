use std::collections::HashMap;

#[allow(unused_variables)]
fn main() {
    let (n, k, target) = (1, 6, 3);
    let (n, k, target) = (2, 6, 7);
    let (n, k, target) = (30, 30, 500);

    println!("Result top-down -> {}", num_rolls_to_target(n, k, target));
    println!(
        "Result bottom-up -> {}",
        num_rolls_to_target_up(n, k, target)
    );
}

/// - Patterns: dp, top-down
/// - Time complexity: $$O(n \cdot k)$$
/// - Space complexity: $$O(n \cdot k)$$
fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
    const MOD: i32 = 10_i32.pow(9) + 7;

    fn dfs(dices: i32, faces: i32, target: i32, cache: &mut HashMap<(i32, i32), i32>) -> i32 {
        if dices == 0 {
            return if target == 0 { 1 } else { 0 };
        }

        if let Some(&v) = cache.get(&(dices, target)) {
            return v;
        }

        let mut result: i32 = 0;
        for f in 1..=faces {
            result = (result + dfs(dices - 1, faces, target - f, cache)) % MOD;
        }

        cache.insert((dices, target), result);

        result
    }

    dfs(n, k, target, &mut HashMap::new())
}

/// - Patterns: dp, bottom-up
/// - Time complexity: $$O(n \cdot k)$$
/// - Space complexity: $$O(target)$$
// NOTE: DP sometimes don't make sense for me :(
fn num_rolls_to_target_up(n: i32, k: i32, target: i32) -> i32 {
    const MOD: i32 = 10_i32.pow(9) + 7;

    let mut dp = vec![0; target as usize + 1];
    dp[0] = 1;

    for _ in 0..n {
        let mut next_dp = vec![0; target as usize + 1];

        for v in 1..=k {
            for total in v as usize..=target as usize {
                next_dp[total] = (next_dp[total] + dp[total - v as usize]) % MOD;
            }
        }

        dp = next_dp;
    }

    dp[target as usize]
}
