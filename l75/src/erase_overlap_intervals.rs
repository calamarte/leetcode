///
/// # [435. Non-overlapping Intervals](https://leetcode.com/problems/non-overlapping-intervals)
///

fn main() {
    let intervals = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]];

    println!("Result -> {}", erase_overlap_intervals(intervals));
}

/// - Patterns: intervals
/// - Time complexity: $$O(n \log n)$$
/// - Space complexity: $$O(1)$$
fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
    if intervals.len() < 2 {
        return 0;
    }

    intervals.sort_by_key(|v| v[0]);

    let mut remove = 0;
    for i in 1..intervals.len() {
        let a_end = intervals[i - 1][1];
        let (b_start, b_end) = (intervals[i][0], intervals[i][1]);

        if b_start < a_end {
            remove += 1;
            intervals[i][1] = a_end.min(b_end);
        }
    }

    remove
}
