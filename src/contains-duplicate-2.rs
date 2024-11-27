use std::{
    collections::{HashMap, HashSet},
    i32,
};

fn main() {
    let nums = (vec![1, 2, 3, 1], 3);
    // let nums = (vec![1, 0, 1, 1], 1);
    // let nums = (vec![1, 2, 3, 1, 2, 3], 2);
    // let nums = (vec![99, 99], 2);
    let result = contains_nearby_duplicate(nums.0, nums.1);

    println!("Result -> {result}");
}

// PERF: Remove one loop
fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    if nums.is_empty() {
        return false;
    }

    let mut window = HashSet::with_capacity((k + 1) as usize);
    let mut left = 0;

    for right in 0..nums.len() {
        if right - left > (k as usize) {
            window.remove(&nums[left]);
            left += 1;
        }

        if window.contains(&nums[right]) {
            return true;
        }

        window.insert(nums[right]);
    }

    false
}

trait Duplicates {
    fn are_duplicates(&self) -> bool;
}

impl Duplicates for [i32] {
    fn are_duplicates(&self) -> bool {
        let mut set = HashSet::with_capacity(self.len());

        for &v in self {
            if !set.insert(v) {
                return true;
            }
        }

        false
    }
}
