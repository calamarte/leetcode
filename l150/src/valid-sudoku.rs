use std::collections::HashSet;

fn main() {
    let board: Vec<Vec<char>> = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    println!("Result -> {}", is_valid_sudoku(board));
}

fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut repeated: HashSet<char> = HashSet::new();

    let mut y_offset = 0;
    let mut x_offset = 0;
    for (i, row) in board.iter().enumerate() {
        // Rows
        if row
            .iter()
            .filter(|&&c| c != '.')
            .any(|&c| !repeated.insert(c))
        {
            return false;
        }

        repeated.clear();

        // Columns
        if (0..9)
            .map(|c_idx| board[c_idx][i])
            .filter(|&c| c != '.')
            .any(|c| !repeated.insert(c))
        {
            return false;
        }

        repeated.clear();

        // Chunks
        println!("Chunk -> {i}:");
        for cy in 0..3 {
            println!("{:?}", &board[cy + y_offset][x_offset..(x_offset + 3)]);
            for cx in 0..3 {
                let cell = board[cy + y_offset][cx + x_offset];

                if cell == '.' {
                    continue;
                }

                if !repeated.insert(cell) {
                    return false;
                }
            }
        }

        if (i + 1) % 3 == 0 {
            y_offset += 3;
            x_offset = 0;
        } else {
            x_offset += 3;
        }

        repeated.clear();
    }

    true
}
