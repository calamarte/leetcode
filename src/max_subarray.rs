#![allow(dead_code)]

#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    // let nums = vec![5, 4, -1, 7, 8];

    println!("Result -> {}", max_sub_array_opt(nums));
}

/// Time complexity: $$O(n^2)$$
/// Space complexity: $$O(1)$$
fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max_sum = i32::MIN;
    for i in 0..nums.len() {
        let mut current_max = nums[i];
        max_sum = current_max.max(max_sum);

        for v in &nums[i + 1..nums.len()] {
            current_max += v;
            max_sum = current_max.max(max_sum);
        }
    }

    max_sum
}

/// # Kadane's algorithm
/// Time complexity: $$O(n)$$
/// Space complexity: $$O(1)$$
fn max_sub_array_opt(nums: Vec<i32>) -> i32 {
    let mut best_sum = i32::MIN;
    let mut current_sum = 0;
    for n in nums {
        current_sum = n.max(current_sum + n);
        best_sum = best_sum.max(current_sum);
    }

    best_sum
}
