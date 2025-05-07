fn main() {
    let digits = "23".to_string();

    println!("Result -> {:?}", letter_combinations(digits));
}

/// - Patterns: backtrack
/// - Time complexity: $$O(4^n)$$
/// - Space complexity: $$O(4^n * n)$$
fn letter_combinations(digits: String) -> Vec<String> {
    const OFFSET: usize = 2;
    const KEYBOARD: [&[char]; 8] = [
        &['a', 'b', 'c'],
        &['d', 'e', 'f'],
        &['g', 'h', 'i'],
        &['j', 'k', 'l'],
        &['m', 'n', 'o'],
        &['p', 'q', 'r', 's'],
        &['t', 'u', 'v'],
        &['w', 'x', 'y', 'z'],
    ];

    if digits.is_empty() {
        return Vec::new();
    }

    fn backtrack(prefix: &mut String, rdigits: &mut Vec<usize>, storage: &mut Vec<String>) {
        if let Some(i) = rdigits.pop() {
            for &c in KEYBOARD[i - OFFSET] {
                prefix.push(c);
                backtrack(prefix, rdigits, storage);
                prefix.pop();
            }

            rdigits.push(i);
        } else {
            storage.push(prefix.to_owned());
        }
    }

    let mut prefix = String::new();
    let mut result = Vec::new();
    let mut rdigits: Vec<usize> = digits
        .chars()
        .map(|v| v.to_digit(10).expect("Value between [2..9]") as usize)
        .rev()
        .collect();

    backtrack(&mut prefix, &mut rdigits, &mut result);

    result
}
