fn main() {
    let (n, k) = (4, 2);

    println!("Result -> {:?}", combine(n, k));
}

// INFO: k is the vec size
fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    fn backtrack(range: &[i32], depth: i32, comb: &mut Vec<i32>, storage: &mut Vec<Vec<i32>>) {
        if depth == 0 {
            storage.push(comb.clone());
            return;
        }

        for (i, &v) in range.iter().enumerate() {
            comb.push(v);
            backtrack(&range[i + 1..], depth - 1, comb, storage);
            comb.pop();
        }
    }

    let mut comb = Vec::new();
    let mut result = Vec::new();
    backtrack(&(1..=n).collect::<Vec<i32>>(), k, &mut comb, &mut result);

    result
}
