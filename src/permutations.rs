fn main() {
    let nums = vec![1, 2, 3];

    println!("Result -> {:?}", permute(nums));
}

/// Time Complexity: $$O(n \cdot n!)$$
/// Space Complexity: $$O(n)$$
fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn backtrack(comb: &mut Vec<i32>, posibilities: &mut Vec<i32>, storage: &mut Vec<Vec<i32>>) {
        if posibilities.is_empty() {
            storage.push(comb.clone());
            return;
        }

        for i in 0..posibilities.len() {
            let value = posibilities.remove(i);

            comb.push(value);
            backtrack(comb, posibilities, storage);
            comb.pop();

            posibilities.insert(i, value);
        }
    }

    let mut combination = Vec::new();
    let mut result = Vec::new();
    backtrack(&mut combination, &mut nums, &mut result);

    result
}
