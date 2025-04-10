#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    #[rustfmt::skip]
    let is_connected = vec![
        vec![1, 1, 0],
        vec![1, 1, 0],
        vec![0, 0, 1]
    ];

    #[rustfmt::skip]
    let is_connected = vec![
        vec![1, 0, 0],
        vec![0, 1, 0],
        vec![0, 0, 1]
    ];

    println!("Result -> {}", find_circle_sum(is_connected));
}

/// - Patterns: DFS
/// - Time complexity: $$O(n^2)$$
/// - Space complexity: $$O(n)$$
fn find_circle_sum(is_connected: Vec<Vec<i32>>) -> i32 {
    fn explore(city_idx: usize, cities: &[Vec<i32>], visited: &mut [bool]) {
        if visited[city_idx] {
            return;
        }

        visited[city_idx] = true;

        for (i, &connection) in cities[city_idx].iter().enumerate() {
            if connection == 1 {
                explore(i, cities, visited);
            }
        }
    }

    let mut visited = vec![false; is_connected.len()];
    let mut provinces = 0;
    for i in 0..is_connected.len() {
        if !visited[i] {
            explore(i, &is_connected, &mut visited);
            provinces += 1;
        }
    }

    provinces
}
