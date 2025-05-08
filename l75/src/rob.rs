#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let nums = vec![1, 2, 3, 1];
    let nums = vec![2, 7, 9, 3, 1];
    let nums = vec![1, 2];
    let nums = vec![2, 1, 1, 2];

    println!("Input -> {nums:?}");
    println!("Result top-down -> {}", rob(nums.clone()));
    println!("Result bottom-up -> {}", rob_up(nums.clone()));
    println!("Result bottom-up-mem -> {}", rob_up_mem(nums.clone()));
}

/// - Patterns: dp, top-down
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
fn rob(nums: Vec<i32>) -> i32 {
    fn dfs(index: usize, houses: &[i32], cache: &mut [i32]) -> i32 {
        if cache[index] != -1 {
            return cache[index];
        }

        let mut max = 0;
        for i in index + 2..houses.len() {
            max = dfs(i, houses, cache).max(max);
        }

        cache[index] = max + houses[index];
        cache[index]
    }

    let cache = &mut vec![-1; nums.len()];
    (0..nums.len()).map(|i| dfs(i, &nums, cache)).max().unwrap()
}

/// - Patterns: dp, bottom-up
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
fn rob_up(nums: Vec<i32>) -> i32 {
    let len = nums.len();

    if len < 3 {
        return nums.into_iter().max().unwrap_or_default();
    }

    let mut dp = vec![0; nums.len()];

    dp[len - 1] = nums[len - 1];
    dp[len - 2] = nums[len - 2].max(nums[len - 1]);

    for i in (0..nums.len() - 2).rev() {
        dp[i] = (nums[i] + dp[i + 2]).max(dp[i + 1]);
    }

    dp[0]
}

/// - Patterns: dp, bottom-up
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(1)$$
fn rob_up_mem(nums: Vec<i32>) -> i32 {
    if nums.len() < 3 {
        return nums.into_iter().max().unwrap_or_default();
    }

    let (mut a, mut b) = (0, 0);

    for v in nums {
        (a, b) = (b, (v + a).max(b));
    }

    b
}
