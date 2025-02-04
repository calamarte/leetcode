use std::collections::HashSet;

#[allow(unused_variables)]
fn main() {
    let n = 4;
    // let n = 1;

    println!("Result -> {}", total_n_queens(n));
}

/// Time complexity: $$O(n!)$$
/// Space complexity: $$O(n)$$
/// Backtracking is hard :(
fn total_n_queens(n: i32) -> i32 {
    fn backtrack(
        queens: usize,
        row: usize,
        trackers: &mut (HashSet<usize>, HashSet<usize>, HashSet<i32>),
        counter: &mut usize,
    ) {
        if queens == row {
            *counter += 1;
            return;
        }

        for c in 0..queens {
            if trackers.0.contains(&c)
                || trackers.1.contains(&(row + c))
                || trackers.2.contains(&(row as i32 - c as i32))
            {
                continue;
            }

            trackers.0.insert(c);
            trackers.1.insert(row + c);
            trackers.2.insert(row as i32 - c as i32);

            backtrack(queens, row + 1, trackers, counter);

            trackers.0.remove(&c);
            trackers.1.remove(&(row + c));
            trackers.2.remove(&(row as i32 - c as i32));
        }
    }

    let col: HashSet<usize> = HashSet::new();
    let diagonal: HashSet<usize> = HashSet::new();
    let neg_diagonal: HashSet<i32> = HashSet::new();
    let mut trackers = (col, diagonal, neg_diagonal);

    let mut result = 0;

    backtrack(n as usize, 0, &mut trackers, &mut result);

    result as i32
}
