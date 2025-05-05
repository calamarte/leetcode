use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_variables)]
fn main() {
    let (nums1, nums2, k) = (vec![1, 3, 3, 2], vec![2, 1, 3, 4], 3);
    // let (nums1, nums2, k) = (vec![4, 2, 3, 1, 1], vec![7, 5, 10, 9, 6], 1);

    println!("Result -> {}", max_score(nums1, nums2, k));
}

/// - Patterns: heap
/// - Time complexity: $$O(n \log n)$$
/// - Space complexity: $$O(n)$$
fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
    let mut pairs: Vec<(i32, i32)> = nums1.into_iter().zip(nums2).collect();

    pairs.sort_by_key(|&(_, v)| Reverse(v));

    let mut min_heap = BinaryHeap::new();
    let mut n1_sum = 0i64;
    let mut result = 0i64;

    for (n1, n2) in pairs {
        n1_sum += n1 as i64;

        min_heap.push(Reverse(n1));

        if min_heap.len() > k as usize {
            n1_sum -= min_heap.pop().unwrap().0 as i64;
        }

        if min_heap.len() == k as usize {
            result = result.max(n1_sum * n2 as i64);
        }
    }

    result
}
