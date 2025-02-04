use std::collections::HashSet;

#[allow(unused_variables)]
fn main() {
    let s = String::from("abcabcbb");
    // let s = String::from("bbbbb");
    let s = String::from("pwwkew");
    // let s = String::from("ynyo");

    let result = length_of_longest_substring(s);
    println!("{result}");
}

fn length_of_longest_substring(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }

    let mut set: HashSet<_> = HashSet::with_capacity(s.len());
    let chars: Vec<_> = s.chars().collect();

    let mut max = 0;
    let mut left = 0;

    for (right_idx, right_c) in chars.iter().enumerate() {
        while !set.insert(right_c) {
            set.remove(&chars[left]); // NOTE:: You have the index to remove
            left += 1;
        }
        max = max.max(right_idx - left + 1);
    }

    if max == 0 {
        1
    } else {
        max as i32
    }
}
