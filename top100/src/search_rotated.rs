///
/// # [33. Search in Rotated Sorted Array](https://leetcode.com/problems/search-in-rotated-sorted-array/)
///
#[allow(unused_variables)]
fn main() {
    let (nums, target) = (vec![4, 5, 6, 7, 0, 1, 2], 0);
    let (nums, target) = (vec![4, 5, 6, 7, 0, 1, 2], 3);
    let (nums, target) = (vec![1], 0);
    let (nums, target) = (vec![3, 1], 3);

    println!("Result -> {}", search(nums, target));
}

/// - Pattern: binary-search
/// - Time complexity: $$O(\log n)$$
/// - Space complexity: $$O(1)$$
fn search(nums: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right) = (0, nums.len());

    while left < right {
        let mid = left + (right - left) / 2;

        if nums[mid] == target {
            return mid as i32;
        }

        let left_is_lower = nums[left] <= nums[mid];

        if left_is_lower {
            if nums[left] <= target && target < nums[mid] {
                right = mid;
            } else {
                left = mid + 1;
            }
        } else if nums[mid] < target && target <= nums[right - 1] {
            left = mid + 1
        } else {
            right = mid;
        }
    }

    -1
}
