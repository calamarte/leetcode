use std::{cmp::max, iter::once, usize};

fn main() {
    let nums = vec![2, 3, 1, 1, 4];
    let nums = vec![2, 3, 0, 1, 4];
    let nums = vec![1, 2];
    let nums = vec![1, 2, 3];
    let result = jumps(nums);

    println!("{result}");
}

fn jumps(nums: Vec<i32>) -> i32 {
    let mut steps = 0;

    let mut distance = 0;

    let mut start = 0;
    while distance < nums.len() - 1 {
        steps += 1;
        let mut new_distance = distance;
        for i in start..=distance {
            new_distance = max(new_distance, i + nums[i] as usize);
        }

        start = distance;
        distance = new_distance;
    }

    steps
}
