#[allow(unused_variables)]
fn main() {
    let (nums, target) = (vec![1, 2, 5, 6], 5);
    let (nums, target) = (vec![1, 3, 5, 6], 7);
    let (nums, target) = (vec![1, 3, 5, 6], 0);

    println!("Result -> {}", search_insert_rust(nums, target));
}

#[allow(dead_code)]
fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let (mut left, mut right) = (0, (nums.len() - 1) as i32);
    while left <= right {
        let middle = (left + right) / 2;

        if target == nums[middle as usize] {
            return middle;
        }

        if target > nums[middle as usize] {
            left = middle + 1;
        } else {
            right = middle - 1;
        }
    }

    left
}

#[allow(dead_code)]
fn search_insert_rust(nums: Vec<i32>, target: i32) -> i32 {
    nums.binary_search(&target).unwrap_or_else(|x| x) as i32
}
