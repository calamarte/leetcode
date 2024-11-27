use std::{iter, usize};

fn main() {
    let words: Vec<String> = vec![
        "Science",
        "is",
        "what",
        "we",
        "understand",
        "well",
        "enough",
        "to",
        "explain",
        "to",
        "a",
        "computer.",
        "Art",
        "is",
        "everything",
        "else",
        "we",
        "do",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect();
    let words: Vec<String> = vec!["Listen", "to", "many,", "speak", "to", "a", "few."]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    // let words: Vec<String> = vec!["a", "b", "c", "d", "e"]
    //     .into_iter()
    //     .map(|s| s.to_string())
    //     .collect();

    let result = full_justify(words, 6);

    println!("{result:#?}");
}

trait StringVec {
    fn space_len(&self) -> usize;
}

impl StringVec for Vec<String> {
    fn space_len(&self) -> usize {
        if self.is_empty() {
            return 0;
        }

        self.iter().map(|s| s.len()).sum::<usize>() + self.len() - 1
    }
}

fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    let width = max_width as usize;
    let mut lines = Vec::new();
    let mut line = Vec::new();

    for w in words {
        if line.space_len() + w.len() + 1 <= width || (line.is_empty() && w.len() == width) {
            line.push(w);
        } else {
            lines.push(line);
            line = vec![w];
        }
    }

    let mut result: Vec<_> = lines.into_iter().map(|l| fill_line(l, width)).collect();

    // last line
    result.push(format!("{:<w$}", line.join(" "), w = width));
    result
}

fn fill_line(mut line: Vec<String>, width: usize) -> String {
    match line.len() {
        0 => return " ".repeat(line.len()),
        1 => return format!("{:<w$}", line[0], w = width),
        _ => (),
    };

    let missing_len = width - line.space_len();
    let matches = line.len() - 1;

    if missing_len <= 0 {
        return line.join(" ");
    }

    if matches == 0 {
        line.push(" ".repeat(missing_len).to_string());
        return line.into_iter().collect();
    }

    let spaces = missing_len / matches;
    let mut remain_iter = iter::repeat(()).take(missing_len % matches);

    println!("missing_len -> {missing_len}");
    println!("matches -> {matches}");
    println!("spaces -> {spaces}");
    println!("remain -> {remain_iter:?}");

    let mut line_str = String::from(&line[0]);
    for i in 1..=matches {
        let mut sp = spaces + 1;
        if let Some(_) = remain_iter.next() {
            sp += 1;
        }

        line_str.push_str(&" ".repeat(sp));
        line_str.push_str(&line[i]);
    }

    line_str
}
