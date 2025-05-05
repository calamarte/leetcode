fn main() {
    let n = 10;

    unsafe {
        println!("Result -> {}", guessNumber(n));
    }
}

/// - Patterns: binarysearch
/// - Time complexity: $$O(\log n)$$
/// - space complexity: $$O(1)$$
#[allow(non_snake_case)]
unsafe fn guessNumber(n: i32) -> i32 {
    let mut left = 1;
    let mut right = n;

    loop {
        let mid = (right - left) / 2;

        match guess(mid) {
            0 => return mid + left,
            -1 => left = mid + left + 1,
            1 => right = mid + left - 1,
        }
    }
}
