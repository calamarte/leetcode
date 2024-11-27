use std::cmp::{max, min};

fn main() {
    // let prices = vec![7, 1, 5, 3, 6, 4];
    let prices = vec![2, 4, 1];
    let result = max_profit(prices);

    println!("{result}");
}

fn max_profit(prices: Vec<i32>) -> i32 {
    let mut low_canditate = i32::MAX;
    let mut profit = 0;

    for price in prices {
        profit = max(profit, price - low_canditate);
        low_canditate = min(price, low_canditate);
    }

    profit
}
