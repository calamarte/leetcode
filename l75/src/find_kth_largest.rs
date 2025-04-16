use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_variables)]
fn main() {
    let (nums, k) = (vec![3, 2, 1, 5, 6, 4], 2);
    let (nums, k) = (vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4);

    println!("Result -> {}", find_kth_largest(nums, k));
}

/// - Patterns: Heap
/// - Time complexity: $$O(n \cdot log n)$$
/// - Space complexity: $$O(n)$$
fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut min_heap = BinaryHeap::with_capacity(k as usize + 1);

    for n in nums {
        min_heap.push(Reverse(n));

        if min_heap.len() > k as usize {
            min_heap.pop();
        }
    }

    min_heap.pop().unwrap().0
}
