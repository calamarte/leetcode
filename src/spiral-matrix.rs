#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    #[rustfmt::skip]
    let matrix = vec![
        vec![1,2,3,4],
        vec![5,6,7,8],
        vec![9, 10, 11,12]
    ];

    #[rustfmt::skip]
    let matrix = vec![
        vec![1,2,3],
        vec![4,5,6],
        vec![7, 8, 9]
    ];

    let result = spiral_order(matrix);

    println!("Result -> {result:?}");
}

#[derive(Clone)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let height = matrix.len();
    let width = matrix[0].len();

    let mut compas = [
        Direction::Right,
        Direction::Down,
        Direction::Left,
        Direction::Up,
    ]
    .into_iter()
    .cycle();

    let mut result: Vec<i32> = Vec::new();
    let mut limit = width;
    let mut step = 0;

    let mut current_direct = compas.next().unwrap();
    let (mut y_limit, mut x_limit) = (height, width);
    let (mut y, mut x) = (0, 0);
    for _ in 0..(height * width) {
        result.push(matrix[y][x]);

        if step + 1 == limit {
            step = 0;

            current_direct = compas.next().unwrap();
            limit = match current_direct {
                Direction::Down | Direction::Up => {
                    y_limit -= 1;
                    y_limit
                }
                _ => {
                    x_limit -= 1;
                    x_limit
                }
            }
        } else {
            step += 1
        }

        match current_direct {
            Direction::Right => x += 1,
            Direction::Down => y += 1,
            Direction::Left => x -= 1,
            Direction::Up => y -= 1,
        }
    }

    result
}
