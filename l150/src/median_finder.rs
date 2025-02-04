fn main() {
    println!("Is a struct!");

    let mut finder = MedianFinder::new();

    finder.add_num(1);
    finder.add_num(2);
    // finder.add_num(3);
    // finder.add_num(4);
    // finder.add_num(5);
    // finder.add_num(6);

    println!("struct -> {finder:?}");
    println!("Median -> {}", finder.find_median());

    finder.add_num(3);

    println!("struct -> {finder:?}");
    println!("Median -> {}", finder.find_median());
}

#[derive(Default, Debug)]
struct MedianFinder {
    values: Vec<i32>,
    cache: Option<f64>,
}

impl MedianFinder {
    fn new() -> Self {
        Default::default()
    }

    fn add_num(&mut self, num: i32) {
        self.values.push(num);
        self.cache = None;
    }

    fn find_median(&mut self) -> f64 {
        let mid = self.values.len() / 2;
        let (_, &mut value, _) = self.values.select_nth_unstable(mid);

        let result = if self.values.len() % 2 != 0 {
            value as f64
        } else {
            let (_, &mut s_val, _) = self.values.select_nth_unstable(mid - 1);
            (value as f64 + s_val as f64) / 2.0
        };

        self.cache.replace(result);
        result
    }
}
