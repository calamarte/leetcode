fn main() {
    let nums = vec![1, 2, 3, 4];
    let nums = vec![-1, 1, 0, -3, 3];
    let result = product_except_self(nums);

    println!("{result:?}");
}

fn product_except_self(mut nums: Vec<i32>) -> Vec<i32> {
    let zeros = nums.iter().filter(|&&x| x == 0).count();

    if zeros > 1 {
        nums.fill(0);
        return nums;
    }

    let total: i32 = nums.iter().filter(|&&x| x != 0).product();
    for n in &mut nums {
        *n = match n {
            0 => total,
            _ if zeros == 0 => total / *n,
            _ => 0,
        }
    }

    nums
}
