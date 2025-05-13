///
/// # [1318. Minimum Flips to Make a OR b Equal to c](https://leetcode.com/problems/minimum-flips-to-make-a-or-b-equal-to-c)
///

#[allow(unused_variables)]
fn main() {
    let (a, b, c) = (2, 6, 5);
    let (a, b, c) = (4, 2, 7);
    let (a, b, c) = (1, 2, 3);

    println!("Result -> {}", min_flips(a, b, c));
}

/// - Patterns: bit
/// - Time complexity: $$O(1)$$
/// - Space complexity: $$O(1)$$
fn min_flips(a: i32, b: i32, c: i32) -> i32 {
    let mut flips = 0;
    for bit in 0..32 {
        let a_bit = (a >> bit) & 1;
        let b_bit = (b >> bit) & 1;
        let c_bit = (c >> bit) & 1;

        if a_bit | b_bit == c_bit {
            continue;
        }

        flips += if c_bit == 1 { 1 } else { a_bit + b_bit };
    }

    flips
}
