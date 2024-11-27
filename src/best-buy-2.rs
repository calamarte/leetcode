fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    let result = max_profit(prices);

    println!("{result}");
}

fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    let mut buy = prices[0];
    let mut days = prices.len();

    for price in prices {
        if buy < price {
            profit += price - buy;
        }
        buy = price;
    }

    profit
}
