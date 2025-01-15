#[allow(unused_variables)]
fn main() {
    let (left, right) = (5, 7);
    let (left, right) = (0, 0);
    let (left, right) = (1, 2147483647);

    println!("Result -> {}", range_bitwise_and(left, right));
}

/// Time complexity: $$O(\log n)$$
/// Space complexity: $$O(1)$$
fn range_bitwise_and(mut left: i32, mut right: i32) -> i32 {
    let mut index = 0;
    while left != right {
        left >>= 1;
        right >>= 1;

        index += 1;
    }

    left << index
}
