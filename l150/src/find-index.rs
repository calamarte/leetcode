fn main() {
    let haystack = String::from("sadbutsad");
    let needle = String::from("sad");

    let result = str_str(haystack, needle);

    println!("{result}");
}

fn str_str(haystack: String, needle: String) -> i32 {
    haystack
        .match_indices(&needle)
        .next()
        .map(|(r, _)| r as i32)
        .unwrap_or(-1)
}
