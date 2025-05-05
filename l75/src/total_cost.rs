use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_variables)]
fn main() {
    let (costs, k, candidates) = (vec![17, 12, 10, 2, 7, 2, 11, 20, 8], 3, 4);
    let (costs, k, candidates) = (vec![1, 2, 4, 1], 3, 3);

    println!("Result -> {}", total_cost(costs, k, candidates));
}

/// - Patterns: heap
/// - Time complexity: $$O(max(candidates, k) * log n)$$
/// - Space complexity: $$O(candidates)$$
fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
    let mut iter = costs.iter();
    let mut min_heap = BinaryHeap::new();

    for _ in 0..candidates {
        if let Some(&v) = iter.next() {
            min_heap.push((Reverse(v), true));
        }

        if let Some(&v) = iter.next_back() {
            min_heap.push((Reverse(v), false));
        }
    }

    let mut result: i64 = 0;
    for _ in 0..k {
        if let Some((Reverse(v), is_front)) = min_heap.pop() {
            result += v as i64;

            let opt = if is_front {
                iter.next()
            } else {
                iter.next_back()
            };

            if let Some(&nv) = opt {
                min_heap.push((Reverse(nv), is_front));
            }
        }
    }

    result
}
