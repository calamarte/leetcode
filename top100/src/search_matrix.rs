///
/// # [74. Search a 2D Matrix](https://leetcode.com/problems/search-a-2d-matrix/)
///
fn main() {
    let (matrix, target) = (
        vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
        3,
    );

    println!("Result -> {}", search_matrix(matrix, target));
}

/// - Patterns: binary-search
/// - Time complexity: $$O(\log (n \cdot m)$$
/// - Space complexity: $$O(1)$$
fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let (rows, columns) = (matrix.len(), matrix[0].len());
    let mut left = 0;
    let mut size = rows * columns;

    while left < size {
        let mid = left + (size - left) / 2;
        let (y, x) = (mid / columns, mid % columns);

        if matrix[y][x] == target {
            return true;
        }

        if matrix[y][x] < target {
            left = mid + 1;
        } else {
            size = mid;
        }
    }

    false
}
