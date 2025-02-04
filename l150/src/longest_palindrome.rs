#![allow(dead_code)]

#[allow(unused_variables)]
fn main() {
    let s = "babad".to_string();
    let s = "cbbd".to_string();

    println!("Result -> {}", longest_palindrome(s));
}

/// Time complexity: $$O(n)$$
/// Space complexity: $$O(1)$$
fn is_palindrome(value: &[u8]) -> bool {
    let (mut left, mut right) = (0, value.len() - 1);
    while left < right {
        if value[left] != value[right] {
            return false;
        }

        left += 1;
        right -= 1;
    }

    true
}

/// Time complexity: $$O(n^3)$$
/// Space complexity: $$O(n)$$
fn longest_palindrome(s: String) -> String {
    let chars: Vec<_> = s.bytes().collect(); // NOTE: Only valid for an ASCII input

    let mut max_palindrome = vec![chars[0]];
    for i in 0..chars.len() {
        let mut value = Vec::with_capacity(chars.len() - i + 1);
        value.push(chars[i]);

        for &c in &chars[i + 1..] {
            value.push(c);

            if value.len() <= max_palindrome.len() {
                continue;
            }

            if is_palindrome(&value) {
                max_palindrome.splice(.., value.clone());
            }
        }

        if value.len() > max_palindrome.len() && is_palindrome(&value) {
            max_palindrome.splice(.., value);
        }
    }

    String::from_utf8(max_palindrome).unwrap()
}

/// Time complexity: $$O(n^2)$$
/// Space complexity: $$O(n)$$
fn longest_palindrome_mid(s: String) -> String {
    let chars: Vec<_> = s.bytes().collect(); // NOTE: Only valid for an ASCII input
    let mut result = Vec::new();

    for i in 0..chars.len() {
        let (mut left, mut right) = (i as isize, i);

        while left >= 0 && right < chars.len() && chars[left as usize] == chars[right] {
            if (right - left as usize + 1) > result.len() {
                result = chars[left as usize..=right].to_vec();
            }

            left -= 1;
            right += 1;
        }

        (left, right) = (i as isize, i + 1);
        while left >= 0 && right < chars.len() && chars[left as usize] == chars[right] {
            if (right - left as usize + 1) > result.len() {
                result = chars[left as usize..=right].to_vec();
            }

            left -= 1;
            right += 1;
        }
    }

    String::from_utf8(result).unwrap()
}
