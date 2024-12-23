fn main() {
    let (candidates, target) = (vec![2, 3, 6, 7], 7);

    println!("Result -> {:?}", combination_sum(candidates, target));
}

// Time complexity: $$O(2^n \cdot k)$$
// Space complexity: $$O(2^n \cdot k)$$
fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn backtrack(
        candidates: &[i32],
        comb: &mut Vec<i32>,
        target: i32,
        storage: &mut Vec<Vec<i32>>,
    ) {
        match comb.iter().sum::<i32>() {
            s if s == target => {
                storage.push(comb.clone());
                return;
            }
            s if s > target || candidates.is_empty() => {
                return;
            }
            _ => (),
        }

        comb.push(candidates[0]);
        backtrack(candidates, comb, target, storage);
        comb.pop();

        backtrack(&candidates[1..], comb, target, storage);
    }

    let mut comb = Vec::new();
    let mut result = Vec::new();
    backtrack(&candidates, &mut comb, target, &mut result);

    result
}
