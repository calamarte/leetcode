use std::collections::HashSet;

#[allow(unused_variables)]
fn main() {
    let (nums1, nums2) = (vec![1, 2, 3], vec![2, 4, 6]);
    let (nums1, nums2) = (vec![1, 2, 3, 3], vec![1, 1, 2, 2]);

    println!("Result -> {:?}", find_difference(nums1, nums2));
}

/// - Patterns: HashSet
/// - Time complexity: $$O(n + m)$$
/// - Space complexity: $$O(n + m)$$
fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    let set1: HashSet<i32> = nums1.into_iter().collect();
    let set2: HashSet<i32> = nums2.into_iter().collect();

    vec![
        set1.difference(&set2).copied().collect(),
        set2.difference(&set1).copied().collect(),
    ]
}
