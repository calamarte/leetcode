use std::cmp::Ordering;

#[allow(unused_variables)]
fn main() {
    #[rustfmt::skip]
    let (matrix, target) = (vec![
        vec![1, 3, 5, 7],
        vec![10, 11, 16, 20],
        vec![23, 30, 34, 60]
    ],
        3
    );

    #[rustfmt::skip]
    let (matrix, target) = (vec![
        vec![1, 3, 5, 7],
        vec![10, 11, 16, 20],
        vec![23, 30, 34, 60]
    ],
        13
    );

    #[rustfmt::skip]
    let (matrix, target) = (vec![
        vec![1, 1],
    ],
        2
    );

    #[rustfmt::skip]
    let (matrix, target) = (vec![
        vec![1],
        vec![3]
    ],
        3
    );

    println!("Result -> {}", search_matrix(matrix, target));
}

fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let n = matrix[0].len();
    let total_len = matrix.len() * n;

    let (mut left, mut right) = (0, (total_len - 1) as i32);

    while left <= right {
        let middle = (left + right) / 2;
        let (y, x) = (middle / n as i32, middle % n as i32);

        // match matrix[y as usize][x as usize] {
        //     v if v > target => right = middle - 1,
        //     v if v < target => left = middle + 1,
        //     _ => return true,
        // }

        match matrix[y as usize][x as usize].cmp(&target) {
            Ordering::Greater => right = middle - 1,
            Ordering::Less => left = middle + 1,
            Ordering::Equal => return true,
        }
    }

    false
}

#[allow(dead_code)]
fn search_matrix_rust(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    matrix
        .into_iter()
        .flatten()
        .collect::<Vec<i32>>()
        .binary_search(&target)
        .map_or(false, |_| true)
}
