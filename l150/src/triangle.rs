#![allow(dead_code)]

use std::collections::HashMap;

#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
    // let triangle = vec![vec![-1], vec![2, 3], vec![1, -1, -3]];

    println!("Result -> {}", minimum_total_up(triangle));
}

/// # Top-down
/// Time complexity: $$O(n^2)$$
/// Space complexity: $$O(n^2)$$
fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    fn dfs(
        depth: usize,
        index: usize,
        triangle: &[Vec<i32>],
        cache: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        if let Some(&v) = cache.get(&(depth, index)) {
            return v;
        }

        if depth == triangle.len() - 1 {
            return triangle[depth][index];
        }

        let left = dfs(depth + 1, index, triangle, cache);
        let right = dfs(depth + 1, index + 1, triangle, cache);

        let result = triangle[depth][index] + left.min(right);
        cache.insert((depth, index), result);
        result
    }

    dfs(0, 0, &triangle, &mut HashMap::new())
}

/// # Bottom-up
/// Time complexity: $$O(n^2)$$
/// Space complexity: $$O(n)$$
fn minimum_total_up(triangle: Vec<Vec<i32>>) -> i32 {
    let mut cache = vec![0; triangle.len() + 1];

    for row in triangle.into_iter().rev() {
        for (i, v) in row.into_iter().enumerate() {
            cache[i] = v + cache[i].min(cache[i + 1]);
        }
    }

    cache[0]
}
