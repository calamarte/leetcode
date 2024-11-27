fn main() {
    let (s, t) = (String::from("abc"), String::from("ahbgdc"));
    let result = is_subsequence(s, t);

    println!("{result}");
}

fn is_subsequence(s: String, t: String) -> bool {
    let mut iter = s.chars();
    let mut to_compare = iter.next();

    if to_compare.is_none() {
        return true;
    }

    for tc in t.chars() {
        match to_compare {
            Some(sc) if sc == tc => to_compare = iter.next(),
            _ => (),
        }

        if to_compare.is_none() {
            return true;
        }
    }

    false
}
