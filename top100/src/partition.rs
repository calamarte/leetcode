///
/// # [131. Palindrome Partitioning](https://leetcode.com/problems/palindrome-partitioning/)
///
fn main() {
    let s = String::from("aab");

    println!("Result -> {:?}", partition(s.clone()));
}

trait Palindrome {
    fn is_palindrome(&self) -> bool;
}

impl Palindrome for String {
    /// - Patterns: two-pointers
    /// - Time complexity: $$O(n)$$
    /// - Space complexity: $$O(1)$$
    fn is_palindrome(&self) -> bool {
        if self.is_empty() {
            return false;
        }

        let target: Vec<_> = self.chars().collect();

        let mut left = 0;
        let mut right = self.len() - 1;
        while left < right {
            if target[left] != target[right] {
                return false;
            }

            left += 1;
            right -= 1;
        }

        true
    }
}

/// - Patterns: Backtrack
/// - Time complexity: $$O(2^n * n)$$
/// - Space complexity: $$O(2^n * n)$$
fn partition(s: String) -> Vec<Vec<String>> {
    fn dfs(
        index: usize,
        substring: &mut String,
        palindromes: &mut Vec<String>,
        used: usize,
        target: &[char],
        storage: &mut Vec<Vec<String>>,
    ) {
        if index >= target.len() {
            if used == target.len() {
                storage.push(palindromes.clone());
            }

            return;
        }

        substring.push(target[index]);

        if substring.is_palindrome() {
            palindromes.push(substring.clone());
            dfs(
                index + 1,
                &mut String::new(),
                palindromes,
                used + substring.len(),
                target,
                storage,
            );
            palindromes.pop();
        }

        dfs(index + 1, substring, palindromes, used, target, storage);

        substring.pop();
    }

    let target: Vec<_> = s.chars().collect();
    let mut result = Vec::new();
    dfs(
        0,
        &mut String::new(),
        &mut Vec::new(),
        0,
        &target,
        &mut result,
    );

    result
}
