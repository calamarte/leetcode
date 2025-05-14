///
/// # [46. Permutations](https://leetcode.com/problems/permutations/)
///
use std::collections::HashSet;

fn main() {
    let nums = vec![1, 2, 3];

    println!("Result -> {:?}", permute(nums));
}

/// - Patterns: bactrack
/// - Time complexity: $$O(n \cdot n!)$$
/// - Space complexity: $$O(n \cdot n!)$$
fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn dfs(
        options: &[i32],
        prefix: &mut Vec<i32>,
        visited: &mut HashSet<usize>,
        storage: &mut Vec<Vec<i32>>,
    ) {
        if options.len() == prefix.len() {
            storage.push(prefix.clone());
        }

        for i in 0..options.len() {
            if visited.contains(&i) {
                continue;
            }

            visited.insert(i);
            prefix.push(options[i]);

            dfs(options, prefix, visited, storage);

            prefix.pop();
            visited.remove(&i);
        }
    }

    let mut result = Vec::new();
    dfs(&nums, &mut Vec::new(), &mut HashSet::new(), &mut result);

    result
}
