#![allow(dead_code)]

fn main() {
    let x = 0b10100101000001111010011100;

    let result = reverse_bits_rust(x);
    println!("Result -> {result}:{result:b}");
}

/// Time complexity: $$O(n)$$
/// Space complexity: $$O(n)$$
fn reverse_bits(x: u32) -> u32 {
    let mut result: u32 = 0;

    for i in 0..32u32 {
        let bit = (x >> i) & 1;
        result |= bit << (31 - i);
    }

    result
}

fn reverse_bits_rust(x: u32) -> u32 {
    x.reverse_bits()
}
