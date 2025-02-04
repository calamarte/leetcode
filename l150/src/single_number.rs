#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let nums = vec![2, 2, 1];
    let nums = vec![4, 1, 2, 1, 2];

    println!("Result -> {}", single_number(nums));
}

/// Time complexity: $$O(n)$$
/// Space complexity: $$O(n)$$
fn single_number(nums: Vec<i32>) -> i32 {
    nums.into_iter().reduce(|acc, v| acc ^ v).unwrap()
}
