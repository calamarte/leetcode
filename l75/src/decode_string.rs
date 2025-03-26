use std::collections::VecDeque;

#[allow(unused_variables)]
fn main() {
    let s = String::from("3[a]2[bc]");
    let s = String::from("3[a2[c]]");
    let s = String::from("2[abc]3[cd]ef");

    println!("Result -> {}", decode_string(s));
}

/// - Pattern: Stack
/// - Time complexity: $$O(k)$$
/// - Space complexity: $$O(k)$$
fn decode_string(s: String) -> String {
    let mut stack: Vec<char> = Vec::new();

    for c in s.chars() {
        if c != ']' {
            stack.push(c);
            continue;
        }

        // PERF: VecDeque don't have **.repeat()**
        let mut expan = Vec::new();
        while let Some(sc) = stack.pop() {
            if sc == '[' {
                break;
            }

            expan.push(sc);
        }

        let mut repeat = VecDeque::new();
        while let Some(&rc) = stack.last() {
            if !rc.is_ascii_digit() {
                break;
            }

            repeat.push_front(rc);
            stack.pop();
        }

        expan.reverse();

        let repeat: usize = repeat.into_iter().collect::<String>().parse().unwrap();
        for exp in expan.repeat(repeat) {
            stack.push(exp);
        }
    }

    stack.into_iter().collect()
}
