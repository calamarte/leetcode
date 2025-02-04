fn main() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let result = three_sum(nums);

    println!("{result:?}");
}

fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();

    nums.sort();

    for (i, &v) in nums.iter().enumerate() {
        if i > 0 && v == nums[i - 1] {
            continue;
        }

        let (mut left, mut right) = (i + 1, nums.len() - 1);

        while left < right {
            match v + nums[left] + nums[right] {
                n if n > 0 => right -= 1,
                n if n < 0 => left += 1,
                _ => {
                    result.push(vec![v, nums[left], nums[right]]);
                    right -= 1;
                    while nums[right] == nums[right + 1] && left < right {
                        right -= 1;
                    }
                }
            };
        }
    }

    result
}
