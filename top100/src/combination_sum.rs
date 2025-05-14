///
/// # [39. Combination Sum](https://leetcode.com/problems/combination-sum)
///

fn main() {
    let (candidates, target) = (vec![2, 3, 6, 7], 7);

    println!("Result -> {:?}", combination_sum(candidates, target));
}

/// - Patterns: backtrack
/// - Time complexity: $$O(1^n \cdot n \log n)$$
/// - Space complexity: $$O(n^2)$$
fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn dfs(
        start: usize,
        candidates: &[i32],
        prefix: &mut Vec<i32>,
        target: i32,
        storage: &mut Vec<Vec<i32>>,
    ) {
        if target < 0 {
            return;
        }

        if target == 0 {
            storage.push(prefix.clone());
            return;
        }

        for i in start..candidates.len() {
            let v = candidates[i];

            prefix.push(v);
            dfs(i, candidates, prefix, target - v, storage);
            prefix.pop();
        }
    }

    candidates.sort();

    let mut result = Vec::new();
    dfs(0, &candidates, &mut Vec::new(), target, &mut result);

    result
}
