fn main() {
    let s = String::from("the sky is blue");
    let result = reverse_words(s);

    println!("{result}");
}

fn reverse_words(s: String) -> String {
    s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
}
