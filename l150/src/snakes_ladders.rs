use std::collections::{HashSet, VecDeque};

fn main() {
    let board = vec![
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, 35, -1, -1, 13, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, 15, -1, -1, -1, -1],
    ];

    println!("Result -> {}", snake_and_ladders(board));
}

trait RealCords {
    fn get_cords(&self, index: i32) -> (usize, usize);
}

impl RealCords for Vec<Vec<i32>> {
    fn get_cords(&self, mut index: i32) -> (usize, usize) {
        let len = self.len() as i32;

        index -= 1;

        let (y, mut x) = (index / len, index % len);

        if y % 2 != 0 {
            x = len - 1 - x;
        }

        (y as usize, x as usize)
    }
}

/// Time complexity: $$O(n^2)$$
/// Space complexity: $$O(n^2)$$
fn snake_and_ladders(mut board: Vec<Vec<i32>>) -> i32 {
    board.reverse();

    let mut deq = VecDeque::new();
    let mut visited = HashSet::new();

    deq.push_front((1, 0));

    while let Some((index, moves)) = deq.pop_back() {
        for i in 1..=6 {
            let mut next_index = index + i;
            let (y, x) = board.get_cords(index + i);

            if board[y][x] != -1 {
                next_index = board[y][x];
            }

            if next_index == board.len().pow(2) as i32 {
                return moves + 1;
            }

            if !visited.contains(&next_index) {
                visited.insert(next_index);
                deq.push_front((next_index, moves + 1));
            }
        }
    }

    -1
}
