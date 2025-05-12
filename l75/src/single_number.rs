///
/// # [136. Single Number](https://leetcode.com/problems/single-number)
///

#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let nums = vec![2, 2, 1];
    let nums = vec![4, 1, 2, 1, 2];

    println!("Result -> {}", single_number(nums));
}

/// - Patterns: bits
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(1)$$
fn single_number(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold(0, |nr, v| nr ^ v)
}
