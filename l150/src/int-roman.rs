use core::panic;

fn main() {
    let num = 3749;
    let num = 58;
    // let num = 3;
    let result = int_to_roman(num);

    println!("{result}");
}

fn int_to_roman(num: i32) -> String {
    let mut num = num;
    let mut result = String::new();

    while num > 0 {
        let first = num.to_string().chars().next().unwrap();

        let (rest, letters) = match first {
            '4' | '9' => substract_sub(num),
            _ => substract(num),
        };

        result.push_str(letters);
        num = rest;
    }

    result
}

fn substract_sub(num: i32) -> (i32, &'static str) {
    match num {
        900.. => (num - 900, "CM"),
        400.. => (num - 400, "CD"),
        90.. => (num - 90, "XC"),
        40.. => (num - 40, "XL"),
        9.. => (num - 9, "IX"),
        4.. => (num - 4, "IV"),
        _ => panic!("Shit.."),
    }
}

fn substract(num: i32) -> (i32, &'static str) {
    match num {
        1000.. => (num - 1000, "M"),
        500.. => (num - 500, "D"),
        100.. => (num - 100, "C"),
        50.. => (num - 50, "L"),
        10.. => (num - 10, "X"),
        5.. => (num - 5, "V"),
        1.. => (num - 1, "I"),
        _ => panic!("Shit..."),
    }
}
