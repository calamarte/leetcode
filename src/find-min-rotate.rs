#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let nums = vec![3, 4, 5, 1, 2];
    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    let nums = vec![11, 13, 15, 17];
    let nums = vec![5, 1, 2, 3, 4];

    println!("Result -> {}", find_min(nums));
}

fn find_min(nums: Vec<i32>) -> i32 {
    let (mut left, mut right) = (0, (nums.len() - 1) as i32);

    let mut min = i32::MAX;
    while left <= right {
        let middle = (left + right) / 2;

        if nums[left as usize] < nums[right as usize] {
            min = min.min(nums[left as usize]);
        }

        min = min.min(nums[middle as usize]);

        if nums[middle as usize] >= nums[left as usize] {
            left = middle + 1;
        } else {
            right = middle - 1;
        }
    }

    min
}
