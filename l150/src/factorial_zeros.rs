#[allow(unused_variables)]
fn main() {
    let n = 3;
    let n = 0;
    let n = 4;
    let n = 5;
    let n = 6;
    let n = 7;
    let n = 8;
    let n = 13;
    let n = 40;

    println!("Result -> {}", trailing_zeroes(n));
}

/// Time complexity: $$O(\log n)$$
/// Space complexity: $$O(1)$$
fn trailing_zeroes(mut n: i32) -> i32 {
    let mut count = 0;
    while n >= 5 {
        n /= 5;
        count += n;
    }

    count
}
