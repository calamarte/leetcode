fn main() {
    let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];

    println!("Result -> {}", length_of_lis(nums));
}

/// # Bottom-up
/// Time complexity: $$O(n^2)$$
/// Space complexity: $$O(n)$$
fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut cache = vec![1; nums.len()];

    for i in (0..nums.len()).rev() {
        for j in i + 1..nums.len() {
            if nums[i] < nums[j] {
                cache[i] = cache[i].max(cache[j] + 1);
            }
        }
    }

    cache.into_iter().max().unwrap()
}
