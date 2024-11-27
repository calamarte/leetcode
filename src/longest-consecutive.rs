use std::collections::HashSet;

fn main() {
    let nums = vec![100, 4, 200, 1, 3, 2];

    println!("{}", longest_consecutive(nums));
}

fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let nums: HashSet<_> = HashSet::from_iter(nums);
    let mut longest = 0;

    for &value in &nums {
        if nums.contains(&(value - 1)) {
            continue;
        }

        let mut len = 0;
        while nums.contains(&(value + len)) {
            len += 1;
        }

        longest = longest.max(len);
    }

    longest
}
