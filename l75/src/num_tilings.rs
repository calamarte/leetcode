/// # [790. Domino and Tromino Tiling](https://leetcode.com/problems/domino-and-tromino-tiling/)

fn main() {
    let n = 3;

    println!("Result bottom-up -> {}", num_tilings_up(n));
}

/// - Patterns: dp, bottom-up
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
// NOTE: WTF???!!! I hate domino! :(
fn num_tilings_up(n: i32) -> i32 {
    const MOD: i32 = 1_000_000_007;

    let n = n as usize;
    if n <= 1 {
        return 1;
    } else if n == 2 {
        return 2;
    }

    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    dp[1] = 1;
    dp[2] = 2;

    for i in 3..=n {
        dp[i] = (2 * dp[i - 1] % MOD + dp[i - 3]) % MOD;
    }

    dp[n]
}
