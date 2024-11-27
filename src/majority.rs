use std::collections::HashMap;

fn main() {
    let nums = vec![3, 2, 3];
    let result = majority_element(nums);

    println!("{result}");
}

// Worst functional solution
fn majority_element(nums: Vec<i32>) -> i32 {
    nums.into_iter()
        .fold(HashMap::new(), |mut acc, v| {
            acc.entry(v).and_modify(|c| *c += 1).or_insert(1);
            acc
        })
        .into_iter()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap()
        .0
}
