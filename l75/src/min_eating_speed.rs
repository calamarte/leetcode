#[allow(unused_variables)]
fn main() {
    let (piles, h) = (vec![3, 6, 7, 11], 8);
    let (piles, h) = (vec![30, 11, 23, 4, 20], 5);

    println!("Result -> {}", min_eating_speed(piles, h));
}

/// - Patterns: binary-search
/// - Time complexity: $$O(\log \text{max}(p) \cdot p)$$
/// - Space complexity: $$O(1)$$
fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let mut left = 0;
    let mut right = piles.iter().max().copied().unwrap_or(0);

    while left < right {
        let mid = left + (right - left) / 2;
        let k = mid + 1;

        let mut time = 0;
        for &p in &piles {
            time += (p as f64 / k as f64).ceil() as i32;
        }

        if time > h {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left + 1
}
