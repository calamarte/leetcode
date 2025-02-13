#[allow(unused_variables)]
fn main() {
    let (s, t) = ("abc".to_string(), "ahbgdc".to_string());
    let (s, t) = ("axc".to_string(), "ahbgdc".to_string());

    println!("Result -> {}", is_subsequence(s, t));
}

/// - Patterns: two pointers
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(1)$$
fn is_subsequence(s: String, t: String) -> bool {
    let mut s_iter = s.chars().peekable();
    for c in t.chars() {
        match s_iter.peek() {
            Some(&sc) if sc == c => {
                s_iter.next();
            }
            None => return true,
            _ => (),
        }
    }

    s_iter.next().is_none()
}
