#![allow(dead_code)]

#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let nums = vec![1, 2, 3, 4];
    // let nums = vec![-1, 1, 0, -3, 3];
    // let nums = vec![0, 0];
    // let nums = vec![0, 4, 0];
    // let nums = vec![1, 0];

    println!("Result -> {:?}", product_except_self_op(nums));
}

/// NOTE: I'm using div (is not allowed)
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(1)$$
fn product_except_self(mut nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() {
        return nums;
    }

    let mut product = 1;
    let mut zeros = 0;
    for &v in &nums {
        if v == 0 {
            zeros += 1;
            continue;
        }

        product *= v;
    }

    if zeros > 1 {
        product = 0;
    }

    for v in &mut nums {
        if *v == 0 {
            *v = product;
            continue;
        }

        *v = if zeros != 0 { 0 } else { product / *v };
    }

    nums
}

/// - Pattern: prefix, postfix
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
/// > NOTE: Space complexity is $$O(1)$$ if you don't count the result allocation
fn product_except_self_op(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![1; nums.len()];

    let mut prefix = 1;
    for i in 0..nums.len() {
        result[i] = prefix;
        prefix *= nums[i];
    }

    let mut suffix = 1;
    for i in (0..nums.len()).rev() {
        result[i] *= suffix;
        suffix *= nums[i];
    }

    result
}
