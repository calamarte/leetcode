#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let asteroids = vec![5, 10, -5];
    let asteroids = vec![8, -8];
    let asteroids = vec![10, 2, -5];
    let asteroids = vec![-2, -1, 1, 2];
    let asteroids = vec![-2, -2, 1, -2];

    println!("Result -> {:?}", asteroids_collision(asteroids));
}

/// - Patterns: Stack
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
fn asteroids_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<i32> = Vec::with_capacity(asteroids.len());

    for astro in asteroids {
        if let Some(&last) = stack.last() {
            if last.signum() == astro.signum() || (last.is_negative() && astro.is_positive()) {
                stack.push(astro);
                continue;
            }

            while !stack.is_empty()
                && (stack.last().unwrap().is_positive() && astro.is_negative())
                && astro.abs() > stack.last().unwrap().abs()
            {
                stack.pop();
            }

            if stack.is_empty() || stack.last().unwrap().signum() == astro.signum() {
                stack.push(astro);
            } else if stack.last().unwrap().abs() == astro.abs() {
                stack.pop();
            }

            continue;
        }

        stack.push(astro);
    }

    stack
}
