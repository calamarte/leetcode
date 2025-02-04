#![allow(dead_code)]

use std::collections::HashMap;

#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let prices = vec![3, 3, 5, 0, 0, 3, 1, 4];
    let prices = vec![1, 2, 3, 4, 5];
    // let prices = vec![7, 6, 4, 3, 1];

    println!("Result -> {}", max_profit_up(prices));
}

/// # Top-down
/// Time complexity: $$O(n)$$
/// Space complexity: $$O(n)$$
/// > NOTE: This shit works but not for leetcode :(
fn max_profit(prices: Vec<i32>) -> i32 {
    fn dfs(
        state: (usize, usize, Option<i32>),
        prices: &[i32],
        cache: &mut HashMap<(usize, usize, Option<i32>), i32>,
    ) -> i32 {
        let (index, transactions, bought) = state;

        if let Some(&r) = cache.get(&state) {
            return r;
        }

        if transactions == 0 || index == prices.len() {
            return 0;
        }

        let result = if let Some(b) = bought {
            dfs((index + 1, transactions, bought), prices, cache)
                .max(prices[index] - b + dfs((index + 1, transactions - 1, None), prices, cache))
        } else {
            dfs((index + 1, transactions, bought), prices, cache).max(dfs(
                (index + 1, transactions, Some(prices[index])),
                prices,
                cache,
            ))
        };

        cache.insert(state, result);

        result
    }

    dfs(
        (0, 2, None),
        &prices,
        &mut HashMap::with_capacity(prices.len().pow(2)),
    )
}

/// # Bottom-up
/// Time complexity: $$O(n)$$
/// Space complexity: $$O(n)$$
/// > NOTE: This code has been copied from leetcode
fn max_profit_up(prices: Vec<i32>) -> i32 {
    let n = prices.len();
    let mut dp = vec![vec![vec![0; 3]; 2]; n + 1];
    for i in (0..n).rev() {
        for b in 0..=1 {
            for t in (0..=2).rev() {
                if t == 2 {
                    dp[i][b][t] = 0;
                    continue;
                }
                if b == 1 {
                    dp[i][b][t] = dp[i + 1][1][t].max(dp[i + 1][0][t] - prices[i]);
                } else {
                    dp[i][b][t] = dp[i + 1][0][t].max(dp[i + 1][1][t + 1] + prices[i]);
                }
            }
        }
    }

    dp[0][1][0]
}
