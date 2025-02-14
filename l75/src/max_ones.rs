#![allow(dead_code)]

#[allow(unused_variables)]
fn main() {
    let (nums, k) = (vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2);
    // let (nums, k) = (
    //     vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
    //     2,
    // );

    println!("Result -> {}", longest_ones_op(nums, k));
}

/// - Patterns: sliding window
/// - Time complexity: $$O(n^2)$$
/// - Space complexity: $$O(1)$$
fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    let mut left = 0;
    let mut right = 0;

    let mut max_ones = 0;
    while right < nums.len() {
        let mut current_ones = 0;
        let mut wildcards = k;

        while right < nums.len() {
            if nums[right] == 1 {
                current_ones += 1;
            } else if wildcards > 0 {
                current_ones += 1;
                wildcards -= 1;
            } else {
                break;
            }

            right += 1;
        }

        max_ones = max_ones.max(current_ones);

        if right >= nums.len() {
            return max_ones;
        }

        left += 1;
        right = left;
    }

    max_ones
}

/// - Patterns: sliding window
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(1)$$
fn longest_ones_op(nums: Vec<i32>, k: i32) -> i32 {
    let mut left = 0;
    let mut right = 0;
    let mut max_ones = 0;
    let mut current_zeros = 0;

    while right < nums.len() {
        if nums[right] == 0 {
            current_zeros += 1;
        }

        while current_zeros > k {
            if nums[left] == 0 {
                current_zeros -= 1;
            }
            left += 1;
        }

        max_ones = max_ones.max(right - left + 1);
        right += 1;
    }

    max_ones as i32
}
