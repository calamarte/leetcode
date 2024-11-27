fn main() {
    let s: String = String::from("A man, a plan, a canal: Panama");
    // let result = is_palindrome(s);
    let result = is_palindrome_2(s);

    println!("{result}");
}

fn is_palindrome(s: String) -> bool {
    let mut clean_s = s.to_lowercase();
    clean_s.retain(|c| c.is_alphanumeric());

    let opposite: String = clean_s.chars().rev().collect();

    opposite == clean_s
}

fn is_palindrome_2(mut s: String) -> bool {
    s = s.to_lowercase();
    let mut iter = s.chars().filter(|c| c.is_alphanumeric());

    while let (Some(s), Some(e)) = (iter.next(), iter.next_back()) {
        if s != e {
            return false;
        }
    }

    true
}
