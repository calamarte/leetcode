///
/// # [72. Edit Distance](https://leetcode.com/problems/edit-distance)
///

fn main() {
    let (word1, word2) = (String::from("horse"), String::from("ros"));

    println!("Result -> {}", min_distance(word1, word2));
}

/// - Patterns: dp, bottom-up
/// - Time complexity: $$O(n \cdot m)$$
/// - Space complexity: $$O(n \cdot m)$$
fn min_distance(word1: String, word2: String) -> i32 {
    let (word1, word2) = (word1.as_bytes(), word2.as_bytes());
    let mut dp = vec![vec![0; word2.len() + 1]; word1.len() + 1];

    #[allow(clippy::needless_range_loop)]
    for i in 0..word1.len() {
        dp[i][word2.len()] = word1.len() - i;
    }

    for i in 0..word2.len() {
        dp[word1.len()][i] = word2.len() - i;
    }

    for y in (0..word1.len()).rev() {
        for x in (0..word2.len()).rev() {
            dp[y][x] = if word1[y] == word2[x] {
                dp[y + 1][x + 1]
            } else {
                dp[y + 1][x].min(dp[y][x + 1]).min(dp[y + 1][x + 1]) + 1
            }
        }
    }

    dp[0][0] as i32
}
