#![allow(dead_code)]

#[allow(unused_variables)]
fn main() {
    let x = 4;
    // let x = 5;
    // let x = 8;
    // let x = 2147483647;

    println!("Result -> {}", my_sqrt(x));
}

/// Time complexity: $$O(\sqrt{n})$$
/// Space complexity: $$O(1)$$
fn my_sqrt(x: i32) -> i32 {
    if x == 0 || x == 1 {
        return x;
    }

    let x = x as u64;
    let mut result = 1;
    for i in 1..x {
        let value = i * i;

        if value == x {
            result = i;
            break;
        }

        if value > x {
            result = i - 1;
            break;
        }
    }

    result as i32
}
