#![allow(dead_code)]

#[allow(unused_variables, unused_mut, clippy::useless_vec)]
fn main() {
    let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
    let mut chars = vec!['a'];
    let mut chars = vec![
        'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
    ];

    println!("Result -> {} - {:?}", compress_op(&mut chars), chars);
}

/// - Patterns: sliding window
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(1)$$
fn compress(chars: &mut Vec<char>) -> i32 {
    const FILL_CHAR: char = '\0';

    let mut left = 0;
    let mut right = 0;

    while left < chars.len() {
        let current = chars[left];

        right += 1;

        let mut count = 1;
        while right < chars.len() && chars[right] == current {
            count += 1;
            right += 1;
        }

        if count == 1 {
            left += 1;
            continue;
        }

        let next_char = right;
        right -= 1;

        while count != 0 {
            let digit = (count % 10) as u8;

            chars[right] = (digit + b'0') as char;

            right -= 1;
            count /= 10;
        }

        chars[left + 1..=right].fill_with(|| FILL_CHAR);

        left = next_char;
        right = next_char;
    }

    chars.retain(|&c| c != FILL_CHAR);
    chars.len() as i32
}

/// - Patterns: sliding window
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(1)$$
fn compress_op(chars: &mut Vec<char>) -> i32 {
    let mut write = 0;
    let mut read = 0;

    while read < chars.len() {
        let current = chars[read];
        let mut count = 0;

        while read < chars.len() && chars[read] == current {
            count += 1;
            read += 1;
        }

        chars[write] = current;
        write += 1;

        if count > 1 {
            for c in count.to_string().chars() {
                chars[write] = c;
                write += 1;
            }
        }
    }

    chars.truncate(write);
    write as i32
}
