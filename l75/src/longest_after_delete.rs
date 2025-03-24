#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let nums = vec![1, 1, 0, 1];
    let nums = vec![0, 1, 1, 1, 0, 1, 1, 0, 1];
    let nums = vec![1, 1, 1];

    println!("Result -> {}", longest_subarray(nums));
}

/// - Patterns: sliding window
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(1)$$
fn longest_subarray(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = 0;

    let mut max = 0;
    let mut zeros = 0;
    while right < nums.len() {
        if nums[right] == 0 {
            zeros += 1;
        }

        while zeros > 1 {
            if nums[left] == 0 {
                zeros -= 1;
            }

            left += 1;
        }

        max = max.max(right - left);

        right += 1;
    }

    max as i32
}
