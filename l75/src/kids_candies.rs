fn main() {
    let (candies, extra) = (vec![2, 3, 5, 1, 3], 3);

    println!("Result -> {:?}", kids_with_candies(candies, extra));
}

/// Time complexity: $$O(n)$$
/// Space complexity: $$O(n)$$
fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max = candies.iter().max().copied().unwrap_or_default();

    candies
        .into_iter()
        .map(|c| c + extra_candies >= max)
        .collect()
}
