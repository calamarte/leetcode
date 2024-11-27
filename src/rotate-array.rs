fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    rotate(&mut nums, 3);

    println!("{nums:?}");
}

fn rotate(nums: &mut Vec<i32>, k: i32) {
    if nums.len() <= 1 {
        return;
    }

    let len = nums.len() as i32;
    let offset = (k % len) as usize;
    println!("{offset}");

    nums.reverse();
    nums[offset..].reverse();
    nums[..offset].reverse();
}
