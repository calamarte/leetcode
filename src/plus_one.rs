#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let digits = vec![1, 2, 3];
    let digits = vec![1, 2, 9];
    let digits = vec![9];

    println!("Result -> {:?}", plus_one(digits));
}

/// Time complexity: $$O(n)$$
/// Space complexity: $$O(1)$$
fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    let last_mut = digits.last_mut().unwrap();
    if *last_mut != 9 {
        *last_mut += 1;
        return digits;
    }

    let mut len = digits.len();
    while let Some(d) = digits.pop() {
        if d != 9 {
            digits.push(d + 1);
            break;
        }
    }

    if digits.is_empty() {
        digits.push(1);
        len += 1;
    }

    digits.resize(len, 0);
    digits
}
