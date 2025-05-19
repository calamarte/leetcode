///
/// # [51. N-Queens](https://leetcode.com/problems/n-queens/)
///
use std::collections::HashSet;

fn main() {
    let n = 4;

    println!("Result -> {:?}", solve_n_queens(n));
}

/// - Patterns: backtrack
/// - Time complexity: $$O(n!)$$
/// - Space complexity: $$O(n^2 \cdot k)$$
fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    fn dfs(
        queens: i32,
        prefix: &mut [Vec<char>],
        (v_y, v_x, v_diag, v_anti_diag): (
            usize,
            &mut HashSet<usize>,
            &mut HashSet<isize>,
            &mut HashSet<usize>,
        ),
        storage: &mut Vec<Vec<String>>,
    ) {
        if queens == 0 {
            let result: Vec<String> = prefix
                .iter()
                .map(|chars| chars.iter().copied().collect())
                .collect();

            storage.push(result);
            return;
        }

        if v_y >= prefix.len() {
            return;
        }

        for x in 0..prefix.len() {
            let (diag, anti_diag) = (v_y as isize - x as isize, v_y + x);

            if !v_x.contains(&x) && !v_diag.contains(&diag) && !v_anti_diag.contains(&anti_diag) {
                v_x.insert(x);
                v_diag.insert(diag);
                v_anti_diag.insert(anti_diag);

                prefix[v_y][x] = 'Q';

                dfs(
                    queens - 1,
                    prefix,
                    (v_y + 1, v_x, v_diag, v_anti_diag),
                    storage,
                );

                v_x.remove(&x);
                v_diag.remove(&diag);
                v_anti_diag.remove(&anti_diag);

                prefix[v_y][x] = '.';
            }
        }
    }

    let mut result = Vec::new();
    dfs(
        n,
        &mut vec![vec!['.'; n as usize]; n as usize],
        (
            0,
            &mut HashSet::new(),
            &mut HashSet::new(),
            &mut HashSet::new(),
        ),
        &mut result,
    );

    result
}
