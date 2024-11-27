use std::{collections::HashMap, i32, usize};

fn main() {
    let nums = vec![2, 7, 11, 15];
    let result = two_sum(nums, 9);

    println!("{result:?}");
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();
    for i in 0..nums.len() {
        if let Some(&j) = map.get(&nums[i]) {
            return {
                let mut l = vec![i as i32 + 1, j as i32 + 1];
                l.sort();
                l
            };
        }

        map.insert(target - nums[i], i);
    }

    Vec::new()
}
