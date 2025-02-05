#[allow(unused_variables)]
fn main() {
    let (str1, str2) = ("ABCABC".to_string(), "ABC".to_string());
    let (str1, str2) = ("ABABAB".to_string(), "ABAB".to_string());
    // let (str1, str2) = ("LEET".to_string(), "CODE".to_string());
    // let (str1, str2) = ("ABCDEF".to_string(), "ABC".to_string());

    println!("Result -> {}", gcd_of_strings(str1, str2));
}

/// Time complexity: $$O(n^2)$$
/// Time complexity: $$O(n)$$
fn gcd_of_strings(str1: String, str2: String) -> String {
    let (len1, len2) = (str1.len(), str2.len());

    let is_divisor = |len: usize| {
        if len1 % len != 0 || len2 % len != 0 {
            return false;
        }

        let pat = &str1[..len];
        pat.repeat(len1 / len) == str1 && pat.repeat(len2 / len) == str2
    };

    for l in (1..=len1.min(len2)).rev() {
        if is_divisor(l) {
            return str1[..l].to_string();
        }
    }

    String::new()
}
