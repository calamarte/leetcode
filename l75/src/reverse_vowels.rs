#![allow(dead_code)]

#[allow(unused_variables)]
fn main() {
    let s = "IceCreAm".to_string();
    let s = "a.".to_string();

    println!("Result -> {}", reverse_vowels_op(s));
}

/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
fn reverse_vowels(s: String) -> String {
    const VOWELS: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let mut word: Vec<_> = s.chars().collect();

    let mut vowels: Vec<_> = word
        .iter()
        .filter(|c| VOWELS.contains(c))
        .copied()
        .collect();

    for c in &mut word {
        if VOWELS.contains(c) {
            *c = vowels.pop().unwrap();
        }
    }

    word.into_iter().collect()
}
/// - Patterns: 2pointers
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(1)$$
fn reverse_vowels_op(mut s: String) -> String {
    const VOWELS: [u8; 10] = [b'a', b'e', b'i', b'o', b'u', b'A', b'E', b'I', b'O', b'U'];
    let word = unsafe { s.as_bytes_mut() }; // WARN: unsafe because are ascii chars

    let mut left = 0;
    let mut right = word.len().saturating_sub(1);

    while left < right {
        while left < right && !VOWELS.contains(&word[left]) {
            left += 1;
        }

        while left < right && !VOWELS.contains(&word[right]) {
            right -= 1;
        }

        if left < right {
            word.swap(left, right);

            left += 1;
            right -= 1;
        }
    }

    s
}
