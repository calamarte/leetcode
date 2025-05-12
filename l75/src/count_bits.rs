//
// # [338. Counting Bits](https://leetcode.com/problems/counting-bits)
//

#[allow(unused_variables)]
fn main() {
    let n = 2;
    let n = 5;

    println!("Result -> {:?}", count_bits(n));
}

/// - Patterns: bits
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
fn count_bits(n: i32) -> Vec<i32> {
    let mut result = Vec::with_capacity(n as usize + 1);
    for i in 0..=n as u32 {
        let mut count = 0;
        for b in 0..32 {
            count += i >> b & 1;
        }

        result.push(count as i32);
    }

    result
}
