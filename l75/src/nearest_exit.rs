use std::collections::{HashSet, VecDeque};

#[allow(unused_variables)]
fn main() {
    let (maze, entrance) = (
        vec![
            vec!['+', '+', '.', '+'],
            vec!['.', '.', '.', '+'],
            vec!['+', '+', '+', '.'],
        ],
        vec![1, 2],
    );

    let (maze, entrance) = (
        vec![
            vec!['+', '+', '+'],
            vec!['.', '.', '.'],
            vec!['+', '+', '+'],
        ],
        vec![1, 0],
    );

    println!("Result -> {}", nearest_exit(maze, entrance));
}

/// - Patterns: BFS
/// - Time complexity: $$O(n \dot m)$$
/// - Space complexity: $$O(n \dot m)$$
/// - Patterns: BFS
fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
    const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

    let mut que = VecDeque::new();
    let mut visited = HashSet::new();

    que.push_front((entrance[0], entrance[1], 0));

    while let Some((y, x, steps)) = que.pop_back() {
        if !visited.insert((y, x)) {
            continue;
        }

        for (dy, dx) in DIRECTIONS {
            let y = y + dy;
            let x = x + dx;

            let in_maze = y < maze.len() as i32 && y >= 0 && x < maze[0].len() as i32 && x >= 0;

            if !in_maze && steps == 0 {
                continue;
            }

            if !in_maze {
                return steps;
            }

            if maze[y as usize][x as usize] == '.' {
                que.push_front((y, x, steps + 1));
            }
        }
    }

    -1
}
