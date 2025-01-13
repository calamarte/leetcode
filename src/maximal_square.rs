#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let matrix = vec![
        vec!['1', '0', '1', '0', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '1', '1', '1', '1'],
        vec!['1', '0', '0', '1', '0'],
    ];

    let matrix = vec![vec!['1', '0'], vec!['1', '0']];
    let matrix = vec![vec!['0', '1']];

    println!("Result -> {}", maximal_square(matrix));
}

/// # Bottom-up
/// Time complexity: $$O(n \cdot m)$$
/// Space complexity: $$O(n \cdot m)$$
fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    let mut cache = vec![vec![0; matrix[0].len() + 1]; matrix.len() + 1];
    let mut max_side = 0;

    for y in (0..matrix.len()).rev() {
        for x in (0..matrix[0].len()).rev() {
            if matrix[y][x] == '0' {
                continue;
            }

            let result = 1 + cache[y][x + 1]
                .min(cache[y + 1][x])
                .min(cache[y + 1][x + 1]);

            max_side = result.max(max_side);

            cache[y][x] = result;
        }
    }

    max_side * max_side
}
