#![allow(dead_code)]

#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let nums = vec![1, 2, 3, 1];
    let nums = vec![2, 7, 9, 3, 1];
    let nums = vec![2, 1, 1, 2];

    println!("Result -> {}", rob_top(nums));
}

/// # Memoization
/// Time complexity $$O(n)$$
/// Space complexity $$O(n)$$
fn rob(nums: Vec<i32>) -> i32 {
    match nums.len() {
        0 => return 0,
        1 => return nums[0],
        2 => return nums.into_iter().max().unwrap(),
        _ => (),
    }

    fn dfs(index: usize, houses: &[i32], memo: &mut [i32]) -> i32 {
        if memo[index] != -1 {
            return memo[index];
        }

        if index >= houses.len() - 2 {
            return houses[index];
        }

        let mut max = 0;
        for i in index + 2..houses.len() {
            max = max.max(dfs(i, houses, memo));
        }

        let result = max + houses[index];
        memo[index] = result;
        result
    }

    let mut memo = vec![-1; nums.len()];
    dfs(0, &nums, &mut memo).max(dfs(1, &nums, &mut memo))
}

/// # Bottom-up
/// Time complexity $$O(n)$$
/// Space complexity $$O(1)$$
fn rob_top(nums: Vec<i32>) -> i32 {
    match nums.len() {
        0 => 0,
        1 => nums[0],
        _ => {
            nums.into_iter()
                .fold((0, 0), |(i1, i2), n| (i1.max(i2 + n), i1))
                .0
        }
    }
}
