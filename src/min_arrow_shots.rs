#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let points = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
    let points = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];
    let points = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];
    let points = vec![
        vec![3, 9],
        vec![7, 12],
        vec![3, 8],
        vec![6, 8],
        vec![9, 10],
        vec![2, 9],
        vec![0, 9],
        vec![3, 9],
        vec![0, 6],
        vec![2, 8],
    ];

    println!("Result -> {}", find_min_arrow_shots(points));
}

/// - Time complexity: $$O(n /log n)$$
/// - Space complexity: $$O(1)$$
fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
    if points.is_empty() {
        return 0;
    }

    points.sort_unstable_by_key(|v| v[0]);

    print_ballons(&points); // WARN: Only for debug

    let mut arrows = 1;
    let mut segment: Option<(i32, i32)> = None;

    for i in 1..points.len() {
        let left: &[i32] = if let Some((s, e)) = segment.take() {
            &[s, e]
        } else {
            points.get(i - 1).unwrap()
        };

        let right = points.get(i).unwrap();

        if right[0] <= left[1] {
            segment = Some((right[0], left[1].min(right[1])));
        } else {
            arrows += 1;
        }
    }

    arrows
}

fn print_ballons(points: &[Vec<i32>]) {
    println!("{:->50}", "");
    for point in points {
        let (s, e) = (point[0], point[1]);
        let start = (s as usize).saturating_sub(1);
        let end = (e as usize - s as usize).saturating_sub(1);

        println!("{:>start$}|{:->end$}| -> {point:?}", "", "");
    }
    println!("{:->50}", "");
}
