#![allow(dead_code)]

#[allow(unused_variables)]
fn main() {
    let (s, word_dict) = (
        "leetcode".to_string(),
        vec!["leet".to_string(), "code".to_string()],
    );

    let (s, word_dict) = (
        "applepenapple".to_string(),
        vec!["apple".to_string(), "pen".to_string()],
    );

    // let (s, word_dict) = (
    //     "catsandong".to_string(),
    //     vec![
    //         "cats".to_string(),
    //         "dog".to_string(),
    //         "sand".to_string(),
    //         "and".to_string(),
    //         "cat".to_string(),
    //     ],
    // );

    println!("Result -> {}", word_break_top(s, word_dict));
}

/// # Memoization
/// Time complexity: $$O(n \cdot m)$$
/// Space complexity: $$O(n)$$
fn word_break(s: String, word_dict: Vec<String>) -> bool {
    fn dfs(index: usize, block: &str, word_dict: &[String], checked: &mut [bool]) -> bool {
        if checked[index] || index > block.len() {
            return false;
        }

        if index == block.len() {
            return true;
        }

        for w in word_dict {
            if block[index..].starts_with(w) {
                continue;
            }

            if dfs(index + w.len(), block, word_dict, checked) {
                return true;
            }
        }

        checked[index] = true;

        false
    }

    dfs(0, &s, &word_dict, &mut vec![false; s.len()])
}

/// # Bottom-up
/// Time complexity: $$O(n \cdot m)$$
/// Space complexity: $$O(n)$$
fn word_break_top(s: String, word_dict: Vec<String>) -> bool {
    let mut cache = vec![false; s.len() + 1];
    cache[s.len()] = true;

    for i in (0..s.len()).rev() {
        for w in &word_dict {
            if cache[i] {
                break;
            }

            if s[i..].starts_with(w) {
                cache[i] = cache[i + w.len()];
            }
        }
    }

    cache[0]
}
