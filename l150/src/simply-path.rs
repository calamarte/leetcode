#[allow(unused_variables)]
fn main() {
    let path = "/home/".to_string();
    let path = "/home//foo".to_string();
    let path = "/home/user/./Documents/../Pictures".to_string();
    let path = "/../home/".to_string();

    println!("{}", simply_path(path));
}

fn simply_path(path: String) -> String {
    let path: Vec<&str> = path
        .trim()
        .split_terminator("/")
        .filter(|&p| !p.is_empty() && p != ".")
        .fold(Vec::new(), |mut acc, p| {
            if p == ".." {
                acc.pop();
            } else {
                acc.push(p);
            }
            acc
        });

    format!("/{}", path.join("/"))
}
