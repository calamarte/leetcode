#[allow(unused_variables)]
fn main() {
    let s = String::from("leet**cod*e");
    let s = String::from("erase*****");

    println!("Result -> {}", remove_starts(s));
}

/// - Pattern: stack
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
fn remove_starts(s: String) -> String {
    let mut stack = String::with_capacity(s.len());

    for c in s.chars() {
        if c == '*' {
            stack.pop();
        } else {
            stack.push(c);
        }
    }

    stack
}
