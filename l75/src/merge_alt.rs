fn main() {
    let (word1, word2) = ("abc".to_string(), "pqr".to_string());

    println!("Result -> {}", merge_alternately(word1, word2));
}

/// Time complexity: $$O(n)$$
/// Space complexity: $$O(n)$$
fn merge_alternately(word1: String, word2: String) -> String {
    let (mut chars1, mut chars2) = (word1.chars().peekable(), word2.chars().peekable());

    let mut result = String::new();
    while chars1.peek().is_some() || chars2.peek().is_some() {
        if let Some(w) = chars1.next() {
            result.push(w);
        }

        if let Some(w) = chars2.next() {
            result.push(w);
        }
    }

    result
}
