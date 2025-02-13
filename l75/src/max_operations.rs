use std::collections::HashMap;

#[allow(unused_variables)]
fn main() {
    let (nums, k) = (vec![1, 2, 3, 4], 5);
    let (nums, k) = (vec![3, 1, 3, 4, 3], 6);
    let (nums, k) = (
        vec![4, 4, 1, 3, 1, 3, 2, 2, 5, 5, 1, 5, 2, 1, 2, 3, 5, 4],
        2,
    );
    let (nums, k) = (vec![2, 2, 2, 3, 1, 1, 4, 1], 4);

    println!("Result -> {}", max_operations(nums, k));
}

/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut freq: HashMap<i32, i32> = HashMap::new();
    let mut operations = 0;

    for n in nums {
        let comp = k - n;

        if let Some(count) = freq.get_mut(&comp) {
            if *count > 0 {
                *count -= 1;
                operations += 1;
                continue;
            }
        }

        freq.entry(n).and_modify(|v| *v += 1).or_insert(1);
    }

    operations
}
