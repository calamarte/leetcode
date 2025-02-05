#[allow(unused_variables)]
fn main() {
    let (flowerbed, n) = (vec![1, 0, 0, 0, 1], 1);
    let (flowerbed, n) = (vec![1, 0, 0, 0, 1], 2);
    let (flowerbed, n) = (vec![1, 0, 0, 0, 0, 1], 1);
    let (flowerbed, n) = (vec![1, 0, 0, 0, 0, 1], 2);

    println!("Result -> {}", can_place_flowers(flowerbed, n));
}

/// # Sliding window
/// Time complexity: $$O(n)$$
/// Space complexity: $$O(1)$$
fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
    let mut left = 0;
    let mut right = 2;

    let mut empty = 0;

    // Edge cases
    if flowerbed[0] == 0 && flowerbed.get(1).copied().unwrap_or_default() == 0 {
        flowerbed[0] = 1;
        empty += 1;
    }

    flowerbed.push(0);

    while right < flowerbed.len() {
        let l = flowerbed[left];
        let r = flowerbed[right];
        let m = flowerbed.get_mut(left + 1).unwrap();

        if *m == 0 && l == 0 && r == 0 {
            *m = 1;
            empty += 1;
        }

        left += 1;
        right += 1;
    }

    n <= empty
}
