///
/// # [452. Minimum Number of Arrows to Burst Balloons](https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons)
///

#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let points = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
    let points = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];
    let points = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];

    println!("Result -> {}", find_min_arrow_shots(points));
}

/// - Patterns: interval
/// - Time complexity: $$O(n \log n)$$
/// - Space complexity: $$O(1)$$
fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
    if points.is_empty() {
        return 0;
    }

    points.sort_by_key(|v| v[0]);

    let mut min_shots = points.len();
    let mut last_end = points[0][1];
    for p in points.into_iter().skip(1) {
        let (start, end) = (p[0], p[1]);

        last_end = if last_end >= start {
            min_shots -= 1;
            end.min(last_end)
        } else {
            end
        };
    }

    min_shots as i32
}
