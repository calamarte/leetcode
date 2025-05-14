///
/// # [739. Daily Temperatures](https://leetcode.com/problems/daily-temperatures/)
///

fn main() {
    let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];

    println!("Result -> {:?}", daily_temperatures(temperatures));
}

/// - Patterns: monotonic-stack
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut result = vec![0; temperatures.len()];
    let mut stack: Vec<(usize, i32)> = Vec::new();
    for (i, t) in temperatures.into_iter().enumerate() {
        while !stack.is_empty() && stack.last().unwrap().1 < t {
            let (st_idx, _) = stack.pop().unwrap();
            result[st_idx] = (i - st_idx) as i32;
        }

        stack.push((i, t));
    }

    result
}
