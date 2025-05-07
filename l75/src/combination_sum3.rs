#[allow(unused_variables)]
fn main() {
    let (k, n) = (3, 7);
    let (k, n) = (3, 9);

    println!("Result -> {:?}", combination_sum3(k, n));
}

/// - Patterns: backtrack
/// - Time complexity: $$O(C(9, k) * k)$$
/// - Space complexity: $$O(C(9, k) * k)$$
fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    fn backtrack(
        prefix: &mut Vec<i32>,
        start: i32,
        remainder: i32,
        combinations: usize,
        visited: &mut [bool],
        storage: &mut Vec<Vec<i32>>,
    ) {
        if prefix.len() > combinations || remainder < 0 {
            return;
        }

        if remainder == 0 && prefix.len() == combinations {
            storage.push(prefix.clone());
            return;
        }

        for v in start..=9 {
            let index = v as usize - 1;
            if visited[index] {
                continue;
            }

            visited[index] = true;
            prefix.push(v);

            backtrack(prefix, v + 1, remainder - v, combinations, visited, storage);

            prefix.pop();
            visited[index] = false;
        }
    }

    let mut prefix = Vec::new();
    let mut result = Vec::new();
    let mut visited = [false; 9];

    backtrack(&mut prefix, 1, n, k as usize, &mut visited, &mut result);

    result
}
