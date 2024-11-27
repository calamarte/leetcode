use std::{collections::HashSet, fs::File, io::Read};

fn main() {
    let mut rset = RandomizedSet::new();
    rset.insert(1);
    rset.insert(2);

    println!("{rset:?}");
    println!("{}", rset.get_random());
    println!("{}", random_number());
}

#[derive(Debug)]
struct RandomizedSet {
    set: HashSet<i32>,
}

impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            set: HashSet::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        self.set.insert(val)
    }

    fn remove(&mut self, val: i32) -> bool {
        self.set.remove(&val)
    }

    fn get_random(&self) -> i32 {
        let random = random_number() % self.set.len();

        *self.set.iter().nth(random).unwrap()
    }
}

fn random_number() -> usize {
    let mut rng = File::open("/dev/urandom").unwrap();
    let mut buff = [0u8; 1];
    rng.read_exact(&mut buff).unwrap();

    buff[0] as usize
}
