///
/// # [4. Median of Two Sorted Arrays](https://leetcode.com/problems/median-of-two-sorted-arrays/)
///
fn main() {
    let (nums1, nums2) = (vec![1, 3], vec![2]);

    println!("Result -> {}", find_median_sorted_arrays(nums1, nums2));
}

/// - Patterns:
/// - Time complexity: $$O(n + m)$$
/// - Space complexity: $$O(n + m)$$
fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut ordered = Vec::with_capacity(nums1.len() + nums2.len());

    let mut i = 0;
    let mut j = 0;
    while i < nums1.len() && j < nums2.len() {
        if nums1[i] < nums2[j] {
            ordered.push(nums1[i]);
            i += 1;
        } else {
            ordered.push(nums2[j]);
            j += 1;
        }
    }

    ordered.extend(&nums1[i..]);
    ordered.extend(&nums2[j..]);

    let mid = ordered.len() / 2;
    if ordered.len() % 2 == 0 {
        (ordered[mid - 1] + ordered[mid]) as f64 / 2.0
    } else {
        ordered[mid] as f64
    }
}
