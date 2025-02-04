use core::num;
use std::{
    collections::{BTreeSet, HashSet},
    iter::Peekable,
};

fn main() {
    let mut nums = vec![1, 1, 2];
    let result = remove_duplicates_set(&mut nums);
    println!("{nums:?}\n{result}");
}

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut iter = nums.iter().peekable();
    let mut result: Vec<i32> = Vec::new();

    while let Some(&val) = iter.next() {
        result.push(val);
        skip(val, &mut iter);
    }

    let len = result.len() as i32;
    *nums = result;

    len
}

fn skip<'a>(value: i32, iter: &mut Peekable<impl Iterator<Item = &'a i32>>) {
    if let Some(&&next) = iter.peek() {
        if next == value {
            match iter.next() {
                Some(&v) => skip(v, iter),
                None => (),
            };
        }
    }
}

fn remove_duplicates_ez(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}

fn remove_duplicates_set(nums: &mut Vec<i32>) -> i32 {
    *nums = nums
        .clone()
        .into_iter()
        .collect::<BTreeSet<i32>>()
        .into_iter()
        .collect::<Vec<i32>>();

    nums.len() as i32
}

fn remove_duplicates_traditional(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut prev_index = 0;
    for i in 1..nums.len() {
        if nums[prev_index] != nums[i] {
            prev_index += 1;
            nums[prev_index] = nums[i];
        }
    }

    nums.len() as i32
}
