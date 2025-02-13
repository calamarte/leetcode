#![allow(dead_code)]

fn main() {
    let (nums, k) = (vec![1, 12, -5, -6, 50, 3], 4);

    println!("Result -> {}", find_max_average_op(nums, k));
}

/// - Patterns: sliding window
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(1)$$
fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let mut max_avg = f64::NEG_INFINITY;
    for window in nums.windows(k as usize) {
        let sum: i32 = window.iter().sum();

        max_avg = max_avg.max(sum as f64 / k as f64);
    }

    if max_avg.is_infinite() {
        return 0.0;
    }

    max_avg
}

/// - Patterns: sliding window
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(1)$$
fn find_max_average_op(nums: Vec<i32>, k: i32) -> f64 {
    let mut left = 0;
    let mut right = k as usize - 1;
    let mut w_sum: i32 = nums[..k as usize].iter().sum();

    let mut max_avg = w_sum as f64 / k as f64;
    while right + 1 < nums.len() {
        w_sum += nums[right + 1];
        w_sum -= nums[left];

        max_avg = max_avg.max(w_sum as f64 / k as f64);

        left += 1;
        right += 1;
    }

    max_avg
}
