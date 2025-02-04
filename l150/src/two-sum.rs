use std::{collections::HashMap, i32, usize};

fn main() {
    let nums = vec![2, 7, 11, 15];
    let result = two_sum_map(nums, 9);

    println!("{result:?}");
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i == j {
                continue;
            }

            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }

    Vec::new()
}

fn two_sum_map(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
    for i in 0..nums.len() {
        if let Some(&j) = map.get(&nums[i]) {
            return vec![i as i32, j as i32];
        }

        map.insert(target - nums[i], i);
    }

    Vec::new()
}
