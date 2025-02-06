fn main() {
    let s = "the sky is blue".to_string();

    println!("Result -> {}", reverse_words(s));
}

/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
fn reverse_words(s: String) -> String {
    s.split_ascii_whitespace()
        .rev()
        .collect::<Vec<_>>()
        .join(" ")
}
