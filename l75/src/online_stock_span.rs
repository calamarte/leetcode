///
/// # [901. Online Stock Span](https://leetcode.com/problems/online-stock-span/)
///

fn main() {
    let mut stock_spanner = StockSpanner::new();

    println!("{}", stock_spanner.next(100));
    println!("{}", stock_spanner.next(80));
    println!("{}", stock_spanner.next(60));
    println!("{}", stock_spanner.next(70));
    println!("{}", stock_spanner.next(60));
    println!("{}", stock_spanner.next(75));
    println!("{}", stock_spanner.next(85));
}

#[derive(Default)]
struct StockSpanner {
    stack: Vec<(i32, i32)>,
}

impl StockSpanner {
    fn new() -> Self {
        Default::default()
    }

    /// - Patterns: monotonic-stack
    /// - Time complexity: $$O(n)$$
    /// - Space complexity: $$O(n)$$
    fn next(&mut self, price: i32) -> i32 {
        let stack = &mut self.stack;

        let mut span = 1;
        while let Some(&(p, s)) = stack.last() {
            if p > price {
                break;
            }

            span += s;
            stack.pop();
        }

        stack.push((price, span));
        span
    }
}
