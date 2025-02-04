#![allow(dead_code)]

#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let nums = vec![1, -2, 3, -2];
    let nums = vec![5, -3, 5];
    let nums = vec![-3, -2, -3];

    println!("Result -> {}", max_subarray_sum_circular_op(nums));
}

/// Time complexity: $$O(n^2)$$
/// Space complexity: $$O(1)$$
fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
    let mut best_sum = i32::MIN;

    for offset in 0..nums.len() {
        let mut current_sum = 0;

        for i in 0..nums.len() {
            let idx = (offset + i + 1) % nums.len();

            current_sum = nums[idx].max(current_sum + nums[idx]);
            best_sum = best_sum.max(current_sum);
        }
    }

    best_sum
}

/// Time complexity: $$O(n)$$
/// Space complexity: $$O(1)$$
fn max_subarray_sum_circular_op(nums: Vec<i32>) -> i32 {
    let mut best_sum = i32::MIN;
    let mut worst_sum = i32::MAX;
    let mut current_min = 0;
    let mut current_max = 0;
    let mut total = 0;

    for &v in &nums {
        current_max = v.max(current_max + v);
        best_sum = best_sum.max(current_max);

        current_min = v.min(v + current_min);
        worst_sum = worst_sum.min(current_min);

        total += v;
    }

    if best_sum < 0 {
        return best_sum;
    }

    best_sum.max(total - worst_sum)
}
