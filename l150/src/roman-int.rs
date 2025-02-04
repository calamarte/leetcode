fn main() {
    let s = String::from("III");
    let s = String::from("LVIII");
    let s = String::from("MCMXCIV");
    let result = roman_to_int(s);

    println!("{result}");
}

fn roman_to_int(s: String) -> i32 {
    let mut chars = s.chars();
    let mut last_char = token_translation(chars.next().unwrap());
    let mut result = last_char;

    for c in chars {
        let n = token_translation(c);

        result += if n > last_char { n - last_char * 2 } else { n };

        last_char = n;
    }

    result
}

fn token_translation(token: char) -> i32 {
    match token {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => panic!("Shit"),
    }
}
