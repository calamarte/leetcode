use std::collections::HashSet;

fn main() {
    #[rustfmt::skip]
    let mut matrix = vec![
        vec![1, 1, 1],
        vec![1, 0, 1],
        vec![1, 1, 1]
    ];

    set_zeroes(&mut matrix);

    println!("Result -> {matrix:?}");
}

#[allow(clippy::ptr_arg)]
pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let mut row_set = HashSet::new();
    let mut column_set = HashSet::new();

    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() {
            if matrix[y][x] == 0 {
                column_set.insert(y);
                row_set.insert(x);
            }
        }
    }

    for i in 0..(matrix.len().max(matrix[0].len())) {
        for &y in &column_set {
            if let Some(cell) = matrix[y].get_mut(i) {
                *cell = 0;
            }
        }

        for &x in &row_set {
            if let Some(col) = matrix.get_mut(i) {
                col[x] = 0;
            }
        }
    }
}
