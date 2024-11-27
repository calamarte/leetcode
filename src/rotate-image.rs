fn main() {
    #[rustfmt::skip]
    let mut matrix = vec![
        vec![1,2,3],
        vec![4,5,6],
        vec![7, 8, 9]
    ];

    rotate_no_alloc(&mut matrix);

    println!("{matrix:?}");
}

// NOTE: Bad solutions is using allocation
fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let mut rotation: Vec<Vec<i32>> = Vec::new();
    for x in 0..matrix[0].len() {
        for y in (0..matrix.len()).rev() {
            match rotation.get_mut(x) {
                Some(v) => v.push(matrix[y][x]),
                None => rotation.push(vec![matrix[y][x]]),
            }
        }
    }

    *matrix = rotation;
}

#[allow(clippy::ptr_arg)]
fn rotate_no_alloc(matrix: &mut Vec<Vec<i32>>) {
    let (mut left, mut right) = (0, matrix.len() - 1);

    while left < right {
        for i in 0..(right - left) {
            let (top, bottom) = (left, right);

            let top_left = matrix[top][left + i];

            matrix[top][left + i] = matrix[bottom - i][left];

            matrix[bottom - i][left] = matrix[bottom][right - i];

            matrix[bottom][right - i] = matrix[top + i][right];

            matrix[top + i][right] = top_left;

            right -= 1;
            left += 1;
        }
    }
}
