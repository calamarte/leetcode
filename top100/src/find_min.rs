///
/// # [153. Find Minimum in Rotated Sorted Array](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/)
///
fn main() {
    let nums = vec![3, 4, 5, 1, 2];

    println!("Result -> {}", find_min(nums));
}

/// - Patterns: binary-search
/// - Time complexity: $$O(\log n)$$
/// - Space complexity: $$O(1)$$
fn find_min(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        let mid = left + (right - left) / 2;

        if nums[mid] > nums[right] {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    nums[left]
}
