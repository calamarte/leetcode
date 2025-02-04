use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
    let intervals = vec![vec![1, 4], vec![4, 5]];
    let intervals = vec![vec![1, 4], vec![2, 3]];

    println!("Result -> {:?}", merge(intervals));
}

// INFO: Probably best solution is sorting the vec to avoid use BinaryHeap
/// Using BinaryHeap or sort approach, same complexity $$O(n \log n)$$
fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if intervals.is_empty() {
        return intervals;
    }

    let mut min_heap: BinaryHeap<_> = intervals
        .into_iter()
        .map(|v| Reverse((v[0], v[1])))
        .collect();

    let mut result = Vec::new();
    let (mut start, mut end) = min_heap.pop().unwrap().0;

    while let Some(Reverse((s, e))) = min_heap.pop() {
        if s <= end {
            end = e.max(end);
        } else {
            result.push(vec![start, end]);
            (start, end) = (s, e.max(end));
        }
    }

    result.push(vec![start, end]);

    result
}
