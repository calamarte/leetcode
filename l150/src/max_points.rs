#![allow(dead_code)]

use std::collections::HashMap;

#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
    let points = vec![
        vec![1, 1],
        vec![3, 2],
        vec![5, 3],
        vec![4, 1],
        vec![2, 3],
        vec![1, 4],
    ];
    //
    let points = vec![vec![9, -25], vec![-4, 1], vec![-1, 5], vec![-7, 7]];
    let points = vec![
        vec![-184, -551],
        vec![-105, -467],
        vec![-90, -394],
        vec![-60, -248],
        vec![115, 359],
        vec![138, 429],
        vec![60, 336],
        vec![150, 774],
        vec![207, 639],
        vec![-150, -686],
        vec![-135, -613],
        vec![92, 289],
        vec![23, 79],
        vec![135, 701],
        vec![0, 9],
        vec![-230, -691],
        vec![-115, -341],
        vec![-161, -481],
        vec![230, 709],
        vec![-30, -102],
    ];

    println!("Result -> {}", max_points_nodfs(points));
}

/// Time complexity: $$O(n^2)$$
/// Space complexity: $$O(n^2)$$
/// > NOTE: This thing is overcomplicated and works very slow :(
fn max_points(points: Vec<Vec<i32>>) -> i32 {
    fn dfs(
        index: usize,
        options: &[Vec<i32>],
        direction: Option<i64>,
        visited: &mut [bool],
        cache: &mut HashMap<i64, i32>,
        cache_slopes: &mut HashMap<(usize, usize), i64>,
    ) -> i32 {
        let mut result = 0;
        if let Some(curr_dir) = direction {
            for i in 0..options.len() {
                if visited[i] {
                    continue;
                }

                let &mut sl = cache_slopes
                    .entry((index, i))
                    .or_insert_with(|| slope(&options[index], &options[i]));

                if sl != curr_dir {
                    continue;
                }

                visited[i] = true;

                result = (1 + dfs(i, options, direction, visited, cache, cache_slopes)).max(result);

                visited[i] = false;
            }
        } else {
            for i in 0..options.len() {
                if visited[i] {
                    continue;
                }

                let n_dir = slope(&options[index], &options[i]);

                if let Some(&np) = cache.get(&n_dir) {
                    result = np.max(result);
                }

                visited[i] = true;

                let count = 1 + dfs(i, options, Some(n_dir), visited, cache, cache_slopes);
                cache.insert(n_dir, count);

                visited[i] = false;

                result = count.max(result);
            }
        }

        result
    }

    let mut visited = vec![false; points.len()];
    let mut cache_directions = HashMap::new();
    let mut cache_slopes = HashMap::new();
    let mut result = 0;

    for i in 0..points.len() {
        visited[i] = true;

        result = dfs(
            i,
            &points,
            None,
            &mut visited,
            &mut cache_directions,
            &mut cache_slopes,
        )
        .max(result);

        visited[i] = false;
    }

    result + 1
}

/// Time complexity: $$O(n^2)$$
/// Space complexity: $$O(n)$$
fn max_points_nodfs(points: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    let mut slope_map: HashMap<i64, i32> = HashMap::with_capacity(points.len());
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            *slope_map.entry(slope(&points[i], &points[j])).or_default() += 1;
        }

        result = slope_map
            .values()
            .copied()
            .max()
            .unwrap_or_default()
            .max(result);

        slope_map.clear();
    }

    result + 1
}

/// Time complexity: $$O(1)$$
/// Space complexity: $$O(1)$$
fn slope(p1: &[i32], p2: &[i32]) -> i64 {
    let (y1, x1) = (p1[0], p1[1]);
    let (y2, x2) = (p2[0], p2[1]);

    match (y2 as f64 - y1 as f64) / (x2 as f64 - x1 as f64) {
        n if n.is_infinite() => i64::MAX,
        n => (n * 1_000_000.0) as i64,
    }
}
