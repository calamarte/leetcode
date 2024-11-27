use core::num;
use std::{cmp, collections::VecDeque, i32, ops::AddAssign, usize};

fn main() {
    let (target, nums) = (7, vec![2, 3, 1, 2, 4, 3]);
    let (target, nums) = (15, vec![5, 1, 3, 5, 10, 7, 4, 9, 2, 8]);
    // let (target, nums) = (4, vec![1, 4, 4]);
    // let (target, nums) = (11, vec![1, 1, 1, 1, 1, 1, 1, 1, 1]);
    // let (target, nums) = (11, vec![1, 2, 3, 4, 5]);
    // let (target, nums) = (6, vec![10, 2, 3]);

    let result = min_sub_array_len(target, nums);

    println!("{result}");
}

fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut total = 0;
    let mut left = 0;
    let mut result = usize::MAX;

    for (right_idx, &right_v) in nums.iter().enumerate() {
        total += right_v;

        while total >= target {
            result = result.min(right_idx - left + 1);
            total -= nums[left];
            left += 1;
        }
    }

    if result == usize::MAX {
        0
    } else {
        result as i32
    }
}
