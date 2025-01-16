#[allow(unused_variables)]
fn main() {
    let (x, n) = (2.0, 10);
    // let (x, n) = (2.10, 3);
    // let (x, n) = (2.00, -2);
    // let (x, n) = (0.00001, 2147483647);
    // let (x, n) = (2.0, -2147483648);

    println!("Result -> {}", my_pow(x, n));
}

/// Time complexity: $$O(\log n)$$
/// Space complexity: $$O(1)$$
fn my_pow(x: f64, n: i32) -> f64 {
    if n == 0 {
        return 1.0;
    }

    let mut result = 1.0;
    let mut base = x;
    let mut exp = n as i64;

    if exp < 0 {
        base = 1.0 / base;
        exp = exp.abs();
    }

    while exp > 0 {
        if exp % 2 == 1 {
            result *= base;
        }

        base *= base;
        exp /= 2;
    }

    result
}
