#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let grid = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];

    let grid = vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1'],
    ];

    println!("Result -> {}", num_islands(grid));
}

/// Time complexity: $$O(n /cdot m)$$
/// Space complexity: $$O(n /cdot m)$$
fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    fn dfs(grid: &mut [Vec<char>], cord: (usize, usize)) {
        const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        let (h, w) = (grid.len(), grid[0].len());
        let (y, x) = cord;
        for (dy, dx) in DIRECTIONS {
            let ny = y as isize + dy;
            let nx = x as isize + dx;

            if ny < 0
                || nx < 0
                || ny as usize >= h
                || nx as usize >= w
                || grid[ny as usize][nx as usize] != '1'
            {
                continue;
            }

            let cord = (ny as usize, nx as usize);
            grid[cord.0][cord.1] = '#';
            dfs(grid, cord);
        }
    }

    let mut counter = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '1' {
                grid[y][x] = '#';
                dfs(&mut grid, (y, x));

                counter += 1;
            }
        }
    }

    counter
}
