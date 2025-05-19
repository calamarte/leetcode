///
/// # [78. Subsets](https://leetcode.com/problems/subsets/)
///

fn main() {
    let nums = vec![1, 2, 3];

    println!("Result -> {:?}", subsets(nums));
}

/// - Patterns: Backtrack
/// - Time complexity: $$O(2^n * n)$$
/// - Space complexity: $$O(2^n * n)$$
fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn dfs(index: usize, nums: &[i32], prefix: &mut Vec<i32>, storage: &mut Vec<Vec<i32>>) {
        if index == nums.len() {
            storage.push(prefix.clone());

            return;
        }

        prefix.push(nums[index]);
        dfs(index + 1, nums, prefix, storage);
        prefix.pop();

        dfs(index + 1, nums, prefix, storage);
    }

    let mut result = Vec::new();
    dfs(0, &nums, &mut Vec::new(), &mut result);

    result
}
