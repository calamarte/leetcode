#![allow(dead_code)]

#[allow(unused_variables)]
fn main() {
    let (intervals, new_interval) = (vec![vec![1, 3], vec![6, 9]], vec![2, 5]);
    let (intervals, new_interval) = (
        vec![
            vec![1, 2],
            vec![3, 5],
            vec![6, 7],
            vec![8, 10],
            vec![12, 16],
        ],
        vec![4, 7],
    );

    println!("Result -> {:?}", insert(intervals, new_interval));
}

// INFO: Sort isn't necessary intervals begins sorted by start
/// Time complexity: $$O(n /log n)$$
/// Space complexity: $$O(n)$$
fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    intervals.push(new_interval);

    if intervals.len() == 1 {
        return intervals;
    }

    intervals.sort_unstable_by_key(|v| v[0]);

    let first = intervals.first().unwrap();

    let mut result = Vec::new();
    let (mut start, mut end) = (first[0], first[1]);
    for range in intervals.into_iter().skip(1) {
        let (s, e) = (range[0], range[1]);

        if s <= end {
            end = e.max(end);
            continue;
        }

        result.push(vec![start, end]);
        (start, end) = (s, e.max(end));
    }

    result.push(vec![start, end]);

    result
}
