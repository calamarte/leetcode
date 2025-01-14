use std::{collections::HashMap, ops::Neg};

#[allow(unused_variables)]
fn main() {
    let (k, prices) = (2, vec![2, 4, 1]);
    let (k, prices) = (2, vec![3, 2, 6, 5, 0, 3]);

    println!("Result -> {}", max_profit(k, prices));
}

/// # Top-down
/// Time complexity: $$O(n)$$
/// Space complexity: $$O(n)$$
/// > NOTE: This code is based on `best_buy_3`
fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
    fn dfs(
        state: (usize, usize, bool),
        prices: &[i32],
        cache: &mut HashMap<(usize, usize, bool), i32>,
    ) -> i32 {
        let (index, transactions, holding) = state;

        if transactions == 0 || index == prices.len() {
            return 0;
        }

        if let Some(&r) = cache.get(&state) {
            return r;
        }

        let skip = dfs((index + 1, transactions, holding), prices, cache);

        let result = if holding {
            let sell = prices[index] + dfs((index + 1, transactions - 1, false), prices, cache);
            skip.max(sell)
        } else {
            let buy = prices[index].neg() + dfs((index + 1, transactions, true), prices, cache);
            skip.max(buy)
        };

        cache.insert(state, result);

        result
    }

    dfs(
        (0, k as usize, false),
        &prices,
        &mut HashMap::with_capacity(prices.len().pow(2)),
    )
}
