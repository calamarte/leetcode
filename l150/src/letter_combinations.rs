fn main() {
    let digits = String::from("23");

    println!("Result -> {:?}", letter_combinations(digits));
}

/// Time complexity: $$O(k^n n)$$
/// Space complexity: $$O(k^n n)$$
fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return Vec::new();
    }

    const MAPPER: [&str; 8] = ["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
    const OFFSET: usize = 2;

    fn backtrack(prefix: &mut String, digits: &str, storage: &mut Vec<String>) {
        if digits.is_empty() {
            storage.push(prefix.to_string());
            return;
        }

        let index = digits.chars().next().unwrap().to_digit(10).unwrap() as usize - OFFSET;
        for c in MAPPER[index].chars() {
            prefix.push(c);
            backtrack(prefix, &digits[1..], storage);
            prefix.pop();
        }
    }

    let mut result = Vec::new();
    let mut prefix = String::new();
    backtrack(&mut prefix, &digits, &mut result);

    result
}
