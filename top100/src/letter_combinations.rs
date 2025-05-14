///
/// # [17. Letter Combinations of a Phone Number](https://leetcode.com/problems/letter-combinations-of-a-phone-number/)
///

fn main() {
    let digits = "23".to_string();

    println!("Result -> {:?}", letter_combinations(digits));
}

/// - Patterns: backtrack
/// - Time complexity: $$O(4^n)$$
/// - Space complexity: $$O(n \cdot 4^n)$$
fn letter_combinations(digits: String) -> Vec<String> {
    const OFFSET: usize = 2;
    const CHARACTERS: [&[char]; 8] = [
        &['a', 'b', 'c'],
        &['d', 'e', 'f'],
        &['g', 'h', 'i'],
        &['j', 'k', 'l'],
        &['m', 'n', 'o'],
        &['p', 'q', 'r', 's'],
        &['t', 'u', 'v'],
        &['w', 'x', 'y', 'z'],
    ];

    if digits.is_empty() {
        return Vec::new();
    }

    fn dfs(index: usize, digits: &[usize], prefix: &mut String, storage: &mut Vec<String>) {
        if index >= digits.len() {
            storage.push(prefix.clone());
            return;
        }

        for &c in CHARACTERS[digits[index] - OFFSET] {
            prefix.push(c);
            dfs(index + 1, digits, prefix, storage);
            prefix.pop();
        }
    }

    let digits: Vec<_> = digits
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let mut result = Vec::new();
    dfs(
        0,
        &digits,
        &mut String::with_capacity(digits.len()),
        &mut result,
    );

    result
}
