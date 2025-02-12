#![allow(dead_code)]

fn main() {
    let mut nums = vec![0, 1, 0, 3, 12];

    move_zeroes_op(&mut nums);

    println!("Result -> {nums:?}");
}

/// - Patterns: two pointers
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(1)$$
#[allow(clippy::ptr_arg)]
fn move_zeroes(nums: &mut Vec<i32>) {
    for i in 0..nums.len() {
        if nums[i] != 0 {
            continue;
        }

        let mut right = i + 1;
        while right < nums.len() && nums[right] == 0 {
            right += 1;
        }

        if right >= nums.len() {
            break;
        }

        nums.swap(i, right);
    }
}

/// - Patterns: two pointers, fast slow
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(1)$$
#[allow(clippy::ptr_arg)]
fn move_zeroes_op(nums: &mut Vec<i32>) {
    let mut write = 0;
    for i in 0..nums.len() {
        if nums[i] == 0 {
            continue;
        }

        nums.swap(write, i);

        write += 1;
    }
}
