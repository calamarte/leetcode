fn main() {
    let n = 11;

    println!("Result -> {}", hamming_weight(n));
}

fn hamming_weight(n: i32) -> i32 {
    let mut count = 0;
    for bit in 0..32 {
        if (n >> bit) & 1 == 1 {
            count += 1;
        }
    }

    count
}
