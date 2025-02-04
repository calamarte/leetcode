use std::i32;

fn main() {
    let mut stack = MinStack::new();

    stack.push(2);
    stack.push(4);
    stack.push(3);
    stack.push(3);
    stack.push(3);
    stack.push(3);

    println!("{stack:?}");
}

#[derive(Debug)]
struct MinStack {
    vec: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        Self {
            vec: Vec::default(),
        }
    }

    fn push(&mut self, val: i32) {
        if self.vec.is_empty() {
            self.vec.push((val, val));
            return;
        }

        let min = val.min(self.vec.last().unwrap().1);
        self.vec.push((val, min));
    }

    fn pop(&mut self) {
        self.vec.pop();
    }

    fn top(&self) -> i32 {
        self.vec.last().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.vec.last().unwrap().1
    }
}
