#[allow(unused_variables)]
fn main() {
    let tokens: Vec<String> = vec![
        "2".to_string(),
        "1".to_string(),
        "+".to_string(),
        "3".to_string(),
        "*".to_string(),
    ];

    let tokens: Vec<String> = vec![
        "4".to_string(),
        "13".to_string(),
        "5".to_string(),
        "/".to_string(),
        "+".to_string(),
    ];

    let tokens: Vec<String> = vec![
        "10".to_string(),
        "6".to_string(),
        "9".to_string(),
        "3".to_string(),
        "+".to_string(),
        "-11".to_string(),
        "*".to_string(),
        "/".to_string(),
        "*".to_string(),
        "17".to_string(),
        "+".to_string(),
        "5".to_string(),
        "+".to_string(),
    ];

    println!("Result -> {}", eval_rpn(tokens));
}

fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut polish: Vec<i32> = Vec::new();

    for token in tokens {
        if let Ok(val) = token.parse() {
            polish.push(val);
            continue;
        }

        let (y, x) = (polish.pop().unwrap(), polish.pop().unwrap());
        match token.as_str() {
            "+" => polish.push(x + y),
            "-" => polish.push(x - y),
            "*" => polish.push(x * y),
            "/" => polish.push(x / y),
            _ => (),
        }
    }

    polish[0]
}
