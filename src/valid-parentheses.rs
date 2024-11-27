use std::collections::{HashMap, HashSet};

#[allow(unused_variables)]
fn main() {
    let s = "()".to_string();
    let s = "()[]{}".to_string();
    let s = "(]".to_string();
    let s = "([])".to_string();

    println!("Result -> {}", is_valid(s));
}

fn is_valid(s: String) -> bool {
    if s.len() % 2 != 0 {
        return false;
    }

    let open_tokens = HashSet::from(['[', '(', '{']);

    #[rustfmt::skip]
    let close_token = HashMap::from([
        (']', '['),
        (')', '('),
        ('}', '{'),
    ]);

    let mut stack: Vec<char> = Vec::with_capacity(s.len());

    for c in s.chars() {
        if open_tokens.contains(&c) {
            stack.push(c);
            continue;
        }

        match (stack.pop(), close_token.get(&c)) {
            (Some(open), Some(&o)) if open == o => (),
            _ => return false,
        }
    }

    stack.is_empty()
}
