use std::collections::{HashMap, HashSet};

#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let arr = vec![1, 2, 2, 1, 1, 3];
    let arr = vec![1, 2];

    println!("Result -> {}", unique_occurrences(arr));
}

/// - Patterns: HashSet, HashMap
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
fn unique_occurrences(arr: Vec<i32>) -> bool {
    let occurrences = arr
        .into_iter()
        .fold(HashMap::<i32, i32>::new(), |mut acc, v| {
            *acc.entry(v).or_default() += 1;
            acc
        })
        .into_values();

    let mut set = HashSet::new();
    for o in occurrences {
        if !set.insert(o) {
            return false;
        }
    }

    true
}
