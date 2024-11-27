use std::{
    collections::{BTreeSet, HashMap, VecDeque},
    ops::Deref,
};

fn main() {
    let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    // let height = vec![4, 2, 0, 3, 2, 5];
    // let height = vec![2, 0, 2];
    // let height = vec![4, 4, 4, 4, 7, 1, 0];
    // let height = vec![0, 7, 1, 4, 6];
    // let height = vec![0, 5, 9, 1, 8, 9, 4, 6, 1];
    let result = trap(height);

    println!("result -> {result}");
}

/**
* ## Strategy 1 (Line per line)
* - Get min height between 2 max heights
* - Trim Vec useless parts
* - loop height by heithg and count holes between them
*
* ## Solution Strategy
*
* $$$
* \begin{array}{rcl}
*     L = \max(l) \\
*    R = \max(r) \\
*    \min(L, R) - h[i] \\
*    \end{array}
* $$$
*
*/

fn trap(height: Vec<i32>) -> i32 {
    if height.len() < 3 {
        return 0;
    }

    height.draw();

    let mut vecl_max = Vec::with_capacity(height.len());
    let mut vecr_max = VecDeque::with_capacity(height.len());

    let mut l_max = 0;
    let mut r_max = 0;
    for (&l, &r) in height.iter().zip(height.iter().rev()) {
        vecl_max.push(l_max);
        vecr_max.push_front(r_max);

        l_max = l_max.max(l);
        r_max = r_max.max(r);
    }

    let mut water = 0;
    for (i, &h) in height.iter().enumerate() {
        water += match i32::min(vecl_max[i], vecr_max[i]) - h {
            v if v > 0 => v,
            _ => 0,
        };
    }

    water as i32
}

trait Graph {
    fn draw(&self);
    fn draw_water(&self, water: &Vec<(i32, i32)>);
}

impl Graph for Vec<i32> {
    fn draw(&self) {
        let &max = self.iter().max().unwrap();
        let len = self.len();

        println!("{:-<len$}", "  ");
        for i in 0..=max {
            let h = i32::abs(max - i);

            print!("|");
            for &hei in self {
                let token = if h >= hei { " " } else { "█" };

                print!("{token}");
            }
            print!("|");

            println!("");
        }
        println!("{:-<len$}", "  ");
        println!("{self:?}");
    }

    fn draw_water(&self, water: &Vec<(i32, i32)>) {
        let &max = self.iter().max().unwrap();
        let len = self.len();

        println!("{:-<len$}", "  ");
        for i in 0..=max {
            let h = i32::abs(max - i);

            print!("|");
            for (idx, &hei) in self.into_iter().enumerate() {
                let token = if h >= hei {
                    let coords = (h, idx as i32);

                    if water.contains(&coords) {
                        "▓"
                    } else {
                        " "
                    }
                } else {
                    "█"
                };

                print!("{token}");
            }
            print!("|");

            println!("");
        }
        println!("{:-<len$}", "  ");
        println!("{self:?}");
        println!("{water:?}");
    }
}
