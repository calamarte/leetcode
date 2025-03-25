use std::collections::HashSet;

fn main() {
    let (word1, word2) = ("abc".to_string(), "bca".to_string());

    println!("Result -> {}", close_strings(word1, word2));
}

/// - Patterns: HashSet
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
fn close_strings(word1: String, word2: String) -> bool {
    if word1.len() != word2.len() {
        return false;
    }

    let mut counts1 = [0u32; 26];
    let mut counts2 = [0u32; 26];

    let mut set1 = HashSet::new();
    let mut set2 = HashSet::new();

    for &c in word1.as_bytes() {
        counts1[(c - b'a') as usize] += 1;
        set1.insert(c);
    }

    for &c in word2.as_bytes() {
        counts2[(c - b'a') as usize] += 1;
        set2.insert(c);
    }

    counts1.sort_unstable();
    counts2.sort_unstable();

    set1 == set2 && counts1 == counts2
}
