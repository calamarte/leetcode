#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let gain = vec![-5, 1, 5, 0, -7];
    let gain = vec![-4, -3, -2, -1, 4, 3, 2];

    println!("Result -> {}", largest_altitude(gain));
}

/// - Patterns: greedy
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(1)$$
fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut altitude = 0;
    let mut high_altitude = 0;

    for g in gain {
        altitude += g;
        high_altitude = high_altitude.max(altitude);
    }

    high_altitude
}
