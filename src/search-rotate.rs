fn main() {
    let (nums, target) = (vec![4, 5, 6, 7, 0, 1, 2], 0);

    println!("{}", search(nums, target));
}

fn search(nums: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right) = (0, (nums.len() - 1) as i32);

    while left <= right {
        let middle = (left + right) / 2;

        if nums[middle as usize] == target {
            return middle;
        }

        if nums[left as usize] <= nums[middle as usize] {
            if target > nums[middle as usize] || target < nums[left as usize] {
                left = middle + 1;
            } else {
                right = middle - 1;
            }
        } else if target <= nums[middle as usize] || target > nums[right as usize] {
            right = middle - 1;
        } else {
            left = middle + 1;
        }
    }

    -1
}

fn search_rust(nums: Vec<i32>, target: i32) -> i32 {
    nums.partition_point(|&x| x > target) as i32
}
