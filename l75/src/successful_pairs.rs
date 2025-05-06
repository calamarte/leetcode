#[allow(unused_variables)]
fn main() {
    let (spells, potions, success) = (vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7);
    // let (spells, potions, success) = (vec![3, 1, 2], vec![8, 5, 8], 16);

    println!("Result -> {:?}", success_pairs(spells, potions, success));
}

/// - Patterns: binary-search
/// - Time complexity: $$O(s \log p + p \log p)$$
/// - Space complexity: $$O(s)$$
fn success_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
    let mut pairs = Vec::with_capacity(spells.len());

    potions.sort();

    for s in spells {
        let mut left = 0;
        let mut right = potions.len();

        while left < right {
            let mid = left + (right - left) / 2;

            if (potions[mid] as i64 * s as i64) < success {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        pairs.push(potions.len() as i32 - left as i32);
    }

    pairs
}
