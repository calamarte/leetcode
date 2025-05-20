///
/// # [35. Search Insert Position](https://leetcode.com/problems/search-insert-position/)
///
#[allow(unused_variables)]
fn main() {
    let (nums, target) = (vec![1, 3, 5, 6], 5);
    let (nums, target) = (vec![1, 3, 5, 6], 2);
    let (nums, target) = (vec![1, 3, 5, 6], 7);

    println!("Result -> {}", search_insert(nums, target));
}

/// - Patterns: binary-search
/// - Time complexity: $$O(\log n)$$
/// - Time complexity: $$O(1)$$
fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len();
    while left < right {
        let mid = left + (right - left) / 2;

        if nums[mid] == target {
            return mid as i32;
        }

        if nums[mid] > target {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left as i32
}
