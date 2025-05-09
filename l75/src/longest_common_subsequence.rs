///
/// # [1143. Longest Common Subsequence](https://leetcode.com/problems/longest-common-subsequence)
///
use std::collections::HashMap;

#[allow(unused_variables)]
fn main() {
    let (text1, text2) = (String::from("abcde"), String::from("ace"));
    let (text1, text2) = (String::from("abc"), String::from("abc"));
    let (text1, text2) = (String::from("abc"), String::from("def"));
    let (text1, text2) = (String::from("bsbininm"), String::from("jmjkbkjkv"));

    println!(
        "Result top-down -> {}",
        longest_common_subsequence(text1.clone(), text2.clone())
    );
    println!(
        "Result bottom-up -> {}",
        longest_common_subsequence_up(text1.clone(), text2.clone())
    );

    println!(
        "Result bottom-up-mem -> {}",
        longest_common_subsequence_up_mem(text1.clone(), text2.clone())
    );
}

/// - Patterns: dp, top-down
/// - Time complexity: $$O(n \cdot m)$$
/// - Space complexity: $$O(n \cdot m)$$
fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    fn dfs(
        cords: (usize, usize),
        texts: (&[u8], &[u8]),
        cache: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        let (idx1, idx2) = cords;

        if idx1 >= texts.0.len() || idx2 >= texts.1.len() {
            return 0;
        }

        if let Some(&c) = cache.get(&cords) {
            return c;
        }

        let result = if texts.0[idx1] == texts.1[idx2] {
            dfs((idx1 + 1, idx2 + 1), texts, cache) + 1
        } else {
            dfs((idx1 + 1, idx2), texts, cache).max(dfs((idx1, idx2 + 1), texts, cache))
        };

        cache.insert(cords, result);

        result
    }

    dfs(
        (0, 0),
        (text1.as_bytes(), text2.as_bytes()), // [a - z]
        &mut HashMap::with_capacity(text1.len() * text2.len()),
    )
}

/// - Patterns: dp, bottom-up
/// - Time complexity: $$O(n \cdot m)$$
/// - Space complexity: $$O(n \cdot m)$$
fn longest_common_subsequence_up(text1: String, text2: String) -> i32 {
    let (text1, text2) = (text1.as_bytes(), text2.as_bytes());
    let mut dp = vec![vec![0; text2.len() + 1]; text1.len() + 1];

    for y in (0..text1.len()).rev() {
        for x in (0..text2.len()).rev() {
            if text1[y] == text2[x] {
                dp[y][x] = dp[y + 1][x + 1] + 1;
            } else {
                dp[y][x] = dp[y + 1][x].max(dp[y][x + 1]);
            }
        }
    }

    dp[0][0]
}

/// - Patterns: dp, bottom-up
/// - Time complexity: $$O(n \cdot m)$$
/// - Space complexity: $$O(m)$$
fn longest_common_subsequence_up_mem(text1: String, text2: String) -> i32 {
    let (text1, text2) = (text1.as_bytes(), text2.as_bytes());
    let mut dp = vec![0; text2.len() + 1];

    for y in (0..text1.len()).rev() {
        let mut diag = 0;

        for x in (0..text2.len()).rev() {
            let tmp = dp[x];

            if text1[y] == text2[x] {
                dp[x] = diag + 1;
            } else {
                dp[x] = dp[x].max(dp[x + 1]);
            }

            diag = tmp;
        }
    }

    dp[0]
}
