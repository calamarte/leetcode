#![allow(dead_code)]
use std::collections::BinaryHeap;

#[allow(unused_variables)]
fn main() {
    let (nums, k) = (vec![3, 2, 1, 5, 6, 4], 2);
    let (nums, k) = (vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4);

    println!("Result -> {}", find_kth_larget_select(nums, k));
}

/// Heap solution (My solution)
/// $$O(n + k \log n)$$
fn find_kth_larget(nums: Vec<i32>, k: i32) -> i32 {
    let mut max_heap = BinaryHeap::from(nums);

    for _ in 0..k - 1 {
        max_heap.pop();
    }

    max_heap.pop().unwrap()
}

/// QuickSelect solution
/// $$O(n)$$
fn find_kth_larget_select(mut nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    *nums.select_nth_unstable(len - k as usize).1
}

/// Dummy solution
/// $$O(n \log n)$$
fn find_kth_larget_sort(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort();
    nums[nums.len() - k as usize]
}
