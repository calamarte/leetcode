fn main() {
    let s = String::from("Today is a nice day");
    let result = length_of_last_word(s);

    println!("{result}");
}

fn length_of_last_word(s: String) -> i32 {
    s.split_whitespace().last().unwrap().len() as i32
}
