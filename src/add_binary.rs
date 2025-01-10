#[allow(unused_variables)]
fn main() {
    let (a, b) = ("11".to_string(), "1".to_string());
    let (a, b) = ("1010".to_string(), "1011".to_string());

    println!("Result -> {}", add_binary(a, b));
}

/// Time complexity: $$O(n)$$
/// Space complexity: $$O(n)$$
fn add_binary(a: String, b: String) -> String {
    let (a, b) = (a.as_bytes(), b.as_bytes());

    let mut result = String::new();
    let mut carry = 0;

    let (mut i, mut j) = (a.len(), b.len());

    #[allow(clippy::char_lit_as_u8)]
    while i > 0 || j > 0 || carry > 0 {
        if i > 0 {
            i -= 1;

            carry += a[i] - '0' as u8;
        }

        if j > 0 {
            j -= 1;
            carry += b[j] - '0' as u8;
        }

        result.push((carry % 2 + '0' as u8) as char);
        carry /= 2;
    }

    unsafe {
        result.as_mut_vec().reverse();
    }

    result
}
