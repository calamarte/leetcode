fn main() {
    let n = 3;

    println!("Result -> {:?}", generate_parenthesis(n));
}

fn generate_parenthesis(n: i32) -> Vec<String> {
    fn backtrack(
        options: usize,
        comb: &mut String,
        state: (usize, usize),
        limit: usize,
        storage: &mut Vec<String>,
    ) {
        if comb.len() == limit {
            storage.push(comb.to_string());
            return;
        }

        let posibles: &[char] = match state {
            (o, _) if o == options => &[')'],
            (_, c) if c == options => &['('],
            (o, c) if o > c => &['(', ')'],
            _ => &['('],
        };

        let (open, close) = state;

        for &c in posibles {
            let n_state = if c == '(' {
                (open + 1, close)
            } else {
                (open, close + 1)
            };

            comb.push(c);
            backtrack(options, comb, n_state, limit, storage);
            comb.pop();
        }
    }

    let mut result = Vec::new();
    backtrack(
        n as usize,
        &mut String::with_capacity(n as usize * 2),
        (0, 0),
        n as usize * 2,
        &mut result,
    );

    result
}
