#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let nums = vec![1, 2, 3, 1];
    let nums = vec![1, 2, 1, 3, 5, 6, 4];
    let nums = vec![1];
    let nums = vec![6, 5, 4, 3, 2, 3, 2];
    println!("Result -> {}", find_peak_element(nums));
}

fn find_peak_element(nums: Vec<i32>) -> i32 {
    if nums.len() <= 2 {
        let &max = nums.iter().max().unwrap();
        return nums.into_iter().position(|x| x == max).unwrap() as i32;
    }

    let (mut left, mut right) = (1, (nums.len() - 2) as i32);
    while left <= right {
        let middle = (left + right) / 2;

        let window = &nums[(middle - 1) as usize..=(middle + 1) as usize];
        let &max = window.iter().max().unwrap();

        if nums[middle as usize] == max {
            return middle;
        }

        if nums[middle as usize] < window[2] {
            left = middle + 1;
        } else {
            right = middle - 1;
        }
    }

    if nums.first().unwrap() > nums.last().unwrap() {
        0
    } else {
        (nums.len() - 1) as i32
    }
}
