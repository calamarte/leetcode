fn main() {
    let strs = vec!["flower", "flow", "flight"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    let result = longest_common_prefix(strs);

    println!("{result}");
}

fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut strs = strs;

    let mut prefix = String::new();
    let sample = strs.pop().unwrap();

    for char in sample.chars() {
        let start = format!("{prefix}{char}");
        let is_valid = strs.iter().all(|s| s.starts_with(&start));

        if !is_valid {
            break;
        }

        prefix = start;
    }

    prefix
}
