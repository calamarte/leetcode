#[allow(unused_variables)]
fn main() {
    let n = 4;
    // let n = 25;
    // let n = 0;

    println!("Result -> {}", tribonacci(n));
    println!("Result tribonacci_pro -> {}", tribonacci_pro(n));
    println!("Result fib -> {}", fibonacci(n));
    println!("Result fibonacci_pro -> {}", fibonacci_pro(n));
}

/// - Patterns: dp
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(1)$$
fn tribonacci(n: i32) -> i32 {
    let mut cells = [0; 38];
    cells[1] = 1;
    cells[2] = 1;

    for i in 0..(n as usize).saturating_sub(2) {
        cells[i + 3] = cells[i] + cells[i + 1] + cells[i + 2];
    }

    cells[n as usize]
}

/// - Patterns: dp
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(1)$$
fn tribonacci_pro(n: i32) -> i32 {
    let (mut a, mut b, mut c) = (0, 1, 1);

    for _ in 3..=n {
        (a, b, c) = (b, c, a + b + c);
    }

    c
}

/// - Patterns: dp
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(n)$$
fn fibonacci(n: i32) -> u128 {
    let mut cells: Vec<u128> = Vec::with_capacity(n as usize + 1);
    cells.push(0);
    cells.push(1);

    for i in 0..(n as usize).saturating_sub(1) {
        cells.push(cells[i] + cells[i + 1]);
    }

    cells[n as usize]
}

/// - Patterns: dp
/// - Time complexity: $$O(n)$$
/// - Space complexity: $$O(1)$$
fn fibonacci_pro(n: i32) -> u128 {
    let (mut a, mut b) = (0, 1);

    for _ in 2..=n {
        (a, b) = (b, a + b);
    }

    b
}
