#[allow(unused_variables)]
fn main() {
    let (s, k) = ("abciiidef".to_string(), 3);
    let (s, k) = ("aeiou".to_string(), 2);
    let (s, k) = ("leetcode".to_string(), 3);

    println!("Result -> {}", max_vowels(s, k));
}

/// - Patterns: sliding window
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(1)$$
fn max_vowels(s: String, k: i32) -> i32 {
    const VOWELS: [u8; 5] = [b'a', b'e', b'i', b'o', b'u'];

    let bytes = s.as_bytes();
    let mut max = bytes[..k as usize]
        .iter()
        .filter(|&c| VOWELS.contains(c))
        .count();

    let mut left = 1;
    let mut right = k as usize;

    let mut current_vowels = max;
    while right < bytes.len() {
        if VOWELS.contains(&bytes[left - 1]) {
            current_vowels -= 1;
        }

        if VOWELS.contains(&bytes[right]) {
            current_vowels += 1;
        }

        max = max.max(current_vowels);

        if max == k as usize {
            return k;
        }

        left += 1;
        right += 1;
    }

    max as i32
}
