///
/// # [34. Find First and Last Position of Element in Sorted Array](https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/)
///
#[allow(unused_variables)]
fn main() {
    let (nums, target) = (vec![5, 7, 7, 8, 8, 10], 8);
    let (nums, target) = (vec![5, 7, 7, 8, 8, 10], 6);
    let (nums, target) = (vec![1], 1);

    println!("Result -> {:?}", search_range(nums, target));
}

/// - Patterns: binary-search
/// - Time complexity $$O(\log n)$$
/// - Space complexity $$O(1)$$
fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let start = find(&nums, target, true);
    let end = find(&nums, target, false);

    match (start.map(|v| v as i32), end.map(|v| v as i32)) {
        (Some(s), Some(e)) if s <= e => vec![s, e],
        _ => vec![-1, -1],
    }
}

fn find(nums: &[i32], target: i32, first: bool) -> Option<usize> {
    let mut left = 0;
    let mut size = nums.len();

    while left < size {
        let mid = left + (size - left) / 2;

        if nums[mid] > target || (first && nums[mid] == target) {
            size = mid;
        } else {
            left = mid + 1;
        }
    }

    if !first {
        left = left.checked_sub(1)?;
    }

    nums.get(left).filter(|&&v| v == target).map(|_| left)
}
