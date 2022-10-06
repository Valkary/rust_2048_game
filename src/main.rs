use rand::Rng;
use std::io::stdin;

fn main() {
    let mut board: [[u32; 4]; 4] = [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    println!("Welcome to the terminal version of 2048 made on Rust");
    
    loop {
        let empty_pos = select_empty_position(&board);
        let (empty_x, empty_y) = empty_pos;
        board[empty_x][empty_y] = 2;

        print_board(&board);

        println!("Insert which move you want to perform next");
        print!("[1] Up\n[2] Down\n[3] Left\n[4] Right\n");

        let mut next_move = String::new();
        stdin().read_line(&mut next_move).expect("Did not input valid move");

        let next_move: u8 = next_move.trim().parse().expect("Did not input number");

        board = move_board(next_move, &board);
    }
}

fn print_board(board: &[[u32; 4]; 4]) {
    let row_separator = "+---+---+---+---+";
    let col_separator = "|";

    for row in 0..board.len() {
        for col in 0..board[row].len() {
            if col % 4 == 0 { println!("{}", row_separator) }
            print!("{} {} ", col_separator, board[row][col]);
            if col == 3 { println!("{}", col_separator) }
            if col == 3 && row == 3 { println!("{}", row_separator) }
        }
    }
}

fn select_empty_position(board: &[[u32; 4]; 4]) -> (usize, usize) {
    let mut rng = rand::thread_rng();

    let mut empty_positions: Vec<(usize, usize)> = vec![];

    for row in 0..board.len() {
        for col in 0..board[row].len() {
            if board[row][col] == 0 { empty_positions.push((row as usize, col as usize)) }
        }
    }

    let random_number: f64 = rng.gen();
    let selected: usize = (random_number * empty_positions.len() as f64).floor() as usize;

    empty_positions[selected]
}

fn move_board(next_move: u8, board: &[[u32; 4]; 4]) -> [[u32; 4]; 4] {
    let mut next_board = board.clone();
    match next_move {
        1 => {
            let size: usize = next_board.len();
            for col in 0..size {
                let mut result_col: [u32; 4] = [0, 0, 0, 0];
                let mut pointer: (usize, u32) = (0, 0);

                for row in 0..size {
                    let (pos, val) = pointer;
                    
                    if next_board[row][col] != 0 {
                        if next_board[row][col] == val {
                            result_col[pos - 1] = val * 2;
                            pointer = (pos, 0);
                        } else {
                            result_col[pos] = next_board[row][col];
                            pointer = (pos + 1, next_board[row][col]);
                        }
                    }
                }

                for row in 0..size {
                    next_board[row][col] = result_col[row];
                }
            }
        },
        2 => {
            let size: usize = next_board.len();
            for col in 0..size {
                let mut result_col: [u32; 4] = [0, 0, 0, 0];
                let mut pointer: (usize, u32) = (size, 0);

                for row in (0..size).rev() {
                    let (pos, val) = pointer;
                    
                    if next_board[row][col] != 0 {
                        if next_board[row][col] == val {
                            result_col[pos] = val * 2;
                            pointer = (pos, 0);
                        } else {
                            result_col[pos - 1] = next_board[row][col];
                            pointer = (pos - 1, next_board[row][col]);
                        }
                    }
                }

                for row in 0..size {
                    next_board[row][col] = result_col[row];
                }
            }
        },
        3 => {
            let size: usize = next_board.len();
            for row in 0..size {
                let mut result_row: [u32; 4] = [0, 0, 0, 0];
                let mut pointer: (usize, u32) = (0, 0);

                for col in 0..size {
                    let (pos, val) = pointer;
                    if next_board[row][col] != 0 {
                        if next_board[row][col] == val {
                            result_row[pos - 1] = val * 2;
                            pointer = (pos, 0);
                        } else {
                            result_row[pos] = next_board[row][col];
                            pointer = (pos + 1, next_board[row][col]);
                        } 
                    }
                }

                next_board[row] = result_row;
            }
        },
        4 => {
            let size: usize = next_board.len();
            for row in 0..size {
                let mut result_row: [u32; 4] = [0, 0, 0, 0];
                let mut pointer: (usize, u32) = (size, 0);

                for col in (0..size).rev() {
                    let (pos, val) = pointer;
                    if next_board[row][col] != 0 {
                        if next_board[row][col] == val {
                            result_row[pos] = val * 2;
                            pointer = (pos, 0);
                        } else {
                            result_row[pos - 1] = next_board[row][col];
                            pointer = (pos - 1, next_board[row][col]);
                        } 
                    }
                }

                next_board[row] = result_row;
            }
        },
        _ => println!("Invalid move"),
    }

    next_board
}
