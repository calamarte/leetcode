fn main() {
    let s = String::from("PAYPALISHIRING");
    let result = convert(s, 4);

    println!("{result}");
}

fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }

    let rows = num_rows as usize;
    let mut canvas: Vec<String> = vec![String::new(); rows];

    let zig_zag = (0..rows).chain((1..rows - 1).rev()).cycle();

    for (i, char) in zig_zag.zip(s.chars()) {
        canvas[i].push(char);
    }

    canvas.into_iter().collect()
}
