///
/// # [1221. Split a String in Balanced Strings](https://leetcode.com/problems/split-a-string-in-balanced-strings/)
///
fn main() {
    let s = "RLRRLLRLRL".to_string();

    println!("Result -> {}", balanced_string_split(s));
}

/// - Patterns: greedy
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(1)$$
fn balanced_string_split(s: String) -> i32 {
    let s = s.as_bytes();

    let (mut r, mut l) = (0, 0);
    let mut sum = 0;
    for &c in s {
        if c == b'R' {
            r += 1;
        } else {
            l += 1;
        }

        if r == l {
            sum += 1;
            (r, l) = (0, 0);
        }
    }

    sum
}
