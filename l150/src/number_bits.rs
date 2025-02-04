#[allow(unused_variables)]
fn main() {
    let n = 11;
    let n = 128;
    let n = 2147483645;

    println!("Result -> {}", hamming_weight(n));
}

// Time complexity: $$O(log n)$$
// Space complexity: $$O(1)$$
fn hamming_weight(mut n: i32) -> i32 {
    let mut counter = n & 1;
    while n != 0 {
        n >>= 1;
        counter += n & 1;
    }

    counter
}
