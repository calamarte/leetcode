#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let height = vec![1, 1];

    println!("Result -> {}", max_area(height));
}

/// - Patterns: two pointers
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(1)$$
fn max_area(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;

    let mut max_area = 0;
    while left < right {
        max_area = max_area.max(height[left].min(height[right]) * (right - left) as i32);

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max_area
}
