use std::{cmp::Ordering, usize, vec};

fn main() {
    let (nums, target) = (vec![5, 7, 7, 8, 8, 10], 8);

    println!("Result -> {:?}", search_range(nums.clone(), target));
    println!("Result binary -> {:?}", search_range_binary(nums, target));
}

fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let (mut left, mut right) = (0, (nums.len() - 1) as i32);

    while left <= right {
        let middle = (left + right) / 2;

        if nums[middle as usize] == target {
            let (mut l, mut r) = (middle - 1, middle + 1);

            while l >= 0 && nums[l as usize] == target {
                l -= 1;
            }

            while r <= (nums.len() - 1) as i32 && nums[r as usize] == target {
                r += 1;
            }

            return vec![l + 1, r - 1];
        }

        if nums[middle as usize] > target {
            right = middle - 1;
        } else {
            left = middle + 1;
        }
    }

    vec![-1, -1]
}

fn search_range_binary(nums: Vec<i32>, target: i32) -> Vec<i32> {
    vec![
        bin_search(&nums, target, true),
        bin_search(&nums, target, false),
    ]
}

fn bin_search(nums: &[i32], target: i32, search_left: bool) -> i32 {
    let (mut left, mut right) = (0, (nums.len() - 1) as i32);

    let mut idx = -1;
    while left <= right {
        let middle = (left + right) / 2;

        match nums[middle as usize].cmp(&target) {
            Ordering::Greater => right = middle - 1,
            Ordering::Less => left = middle + 1,
            Ordering::Equal => {
                idx = middle;

                if search_left {
                    right = middle - 1;
                } else {
                    left = middle + 1;
                }
            }
        }
    }

    idx
}
