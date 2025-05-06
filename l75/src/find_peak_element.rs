#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let nums = vec![1, 2, 3, 1];
    let nums = vec![1, 2, 1, 3, 5, 6, 4];
    let nums = vec![2, 1];
    let nums = vec![1, 2];

    println!("Result -> {}", find_peak_element(nums));
}

/// - Patterns: binary-search
/// - Time complexity: $$O(\log n)$$
/// - Space complexity: $$O(1)$$
fn find_peak_element(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len();

    while left < right {
        let mid = left + (right - left) / 2;

        let left_lower = (mid as isize - 1) < 0 || nums[mid - 1] <= nums[mid];
        let right_lower = (mid + 1) >= nums.len() || nums[mid + 1] <= nums[mid];

        match (left_lower, right_lower) {
            (true, true) => return mid as i32,
            (false, true) => right = mid,
            (_, false) => left = mid + 1,
        }
    }

    left as i32
}
