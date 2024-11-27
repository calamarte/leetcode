fn main() {
    let mut nums = vec![1, 1, 1, 2, 2, 3];
    let result = remove_duplicates(&mut nums);

    println!("{nums:?}\n{result}");
}

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    const MAX_REP: i32 = 2;
    let mut mv = 0;
    let mut rep = (nums[0], 1);

    for i in 1..nums.len() {
        if rep.0 == nums[i] {
            rep.1 += 1;
        } else {
            rep = (nums[i], 1);
        }

        if rep.1 > MAX_REP {
            mv += 1;
        }

        nums[i - mv] = nums[i];
    }

    (nums.len() - mv) as i32
}
