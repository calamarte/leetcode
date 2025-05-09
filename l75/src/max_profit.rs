use std::collections::HashMap;

#[allow(unused_variables)]
fn main() {
    let (prices, fee) = (vec![1, 3, 2, 8, 4, 9], 2);
    // let (prices, fee) = (vec![1, 3, 7, 5, 10, 3], 3);

    println!("Input -> (fee: {fee}) {prices:?}");
    println!("Result top-down -> {}", max_profit(prices.clone(), fee));
    println!("Result bottom-up -> {}", max_profit_up(prices.clone(), fee));
}

/// - Patterns: dp, top-down
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
    fn dfs(
        index: usize,
        prices: &Vec<i32>,
        fee: &i32,
        holding: bool,
        cache: &mut HashMap<(usize, bool), i32>,
    ) -> i32 {
        if index >= prices.len() {
            return 0;
        }

        if let Some(&v) = cache.get(&(index, holding)) {
            return v;
        }

        let result = if holding {
            let sell = dfs(index + 1, prices, fee, false, cache) + prices[index] - fee;
            let hold = dfs(index + 1, prices, fee, holding, cache);

            sell.max(hold)
        } else {
            let buy = dfs(index + 1, prices, fee, true, cache) - prices[index];
            let skip = dfs(index + 1, prices, fee, holding, cache);

            buy.max(skip)
        };

        cache.insert((index, holding), result);
        result
    }

    dfs(0, &prices, &fee, false, &mut HashMap::new())
}

/// - Patterns: dp, bottom-up
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(1)$$
fn max_profit_up(prices: Vec<i32>, fee: i32) -> i32 {
    let mut hold = -prices[0];
    let mut not_hold = 0;

    for p in prices {
        (not_hold, hold) = (not_hold.max(hold + p - fee), hold.max(not_hold - p));
    }

    not_hold
}
