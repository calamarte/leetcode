use std::collections::HashMap;

#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    #[rustfmt::skip]
    let grid = vec![
        vec![3, 2, 1],
        vec![1, 7, 6],
        vec![2, 7, 7]
    ];

    let grid = vec![
        vec![3, 1, 2, 2],
        vec![1, 4, 4, 5],
        vec![2, 4, 2, 2],
        vec![2, 4, 2, 2],
    ];

    println!("Result -> {}", equal_pairs(grid));
}

/// - Patterns: HashMap
/// - Time complexity: $$O(n \cdot m)$$
/// - Space complexity: $$O(n \cdot m)$$
fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
    let row_map = grid.iter().fold(HashMap::new(), |mut map, v| {
        map.entry(v).and_modify(|c| *c += 1).or_insert(1);
        map
    });

    let mut pairs = 0;
    for x in 0..grid[0].len() {
        let column: Vec<_> = grid.iter().map(|r| r[x]).collect();

        if let Some(&p) = row_map.get(&column) {
            pairs += p;
        }
    }

    pairs
}
