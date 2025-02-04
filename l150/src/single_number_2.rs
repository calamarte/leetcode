#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let nums = vec![2, 2, 3, 2];
    // let nums = vec![0, 1, 0, 1, 0, 1, 99];

    println!("Result -> {}", single_number(nums));
}

/// Time complexity: $$O(n)$$
/// Space complexity: $$O(1)$$
fn single_number(nums: Vec<i32>) -> i32 {
    let (mut ones, mut twos) = (0, 0);
    for n in nums {
        ones = (ones ^ n) & !twos;
        twos = (twos ^ n) & !ones;
    }

    ones
}
