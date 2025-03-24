#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let nums = vec![1, 7, 3, 6, 5, 6];
    let nums = vec![1, 2, 3];
    let nums = vec![2, 1, -1];
    let nums = vec![-1, -1, -1, -1, -1, 0];

    println!("Result -> {}", pivot_index(nums));
}

/// - Patterns: prefix-sum
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(1)$$
fn pivot_index(nums: Vec<i32>) -> i32 {
    let total: i32 = nums.iter().sum();
    let mut left = 0;
    let mut right = total;

    for (i, v) in nums.into_iter().enumerate() {
        right -= v;

        if left == right {
            return i as i32;
        }

        left += v;
    }

    -1
}
