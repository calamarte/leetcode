use std::usize;

fn main() {
    // let nums = vec![2, 3, 1, 1, 4];
    let nums = vec![3, 2, 1, 0, 4];
    // let nums = vec![2, 0];
    let result = can_jump(nums);

    println!("{result}");
}

fn can_jump(nums: Vec<i32>) -> bool {
    if nums.len() <= 1 {
        return true;
    }

    let mut max_jump = nums[0];

    for current in nums {
        if max_jump == 0 {
            return false;
        }

        max_jump -= 1;

        if current > max_jump {
            max_jump = current;
        }
    }

    true
}
