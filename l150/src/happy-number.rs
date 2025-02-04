use std::collections::HashSet;

#[allow(unused_variables)]
fn main() {
    let n = 19;
    // let n = 2;
    // let n = 1111111;

    println!("Result -> {}", is_happy_magic(n));
}

fn is_happy(mut n: i32) -> bool {
    if n == 1 {
        return true;
    }

    let mut set: HashSet<i32> = HashSet::new();
    while set.insert(n) {
        let result: u32 = n
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .map(|d| d * d)
            .sum();

        if result == 1u32 {
            return true;
        }

        n = result as i32;
    }

    false
}

fn is_happy_magic(mut n: i32) -> bool {
    if n == 1 {
        return true;
    }

    let mut set: HashSet<i32> = HashSet::new();
    while set.insert(n) {
        println!("{set:?}");

        let mut digits = n;
        let result: i32 = (0..)
            .map(move |_| {
                let digit = digits % 10;
                digits /= 10;
                digit
            })
            .take_while(|&d| d > 0)
            .map(|d| d * d)
            .sum();

        if result == 1 {
            return true;
        }

        n = result;
    }

    false
}
