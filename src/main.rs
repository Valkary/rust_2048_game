use std::io::stdin;

pub mod game_2048 {
    use rand::Rng;

    #[derive(Clone, Copy)]
    pub enum Moves {
        LEFT,
        RIGHT,
        UP,
        DOWN,
        RESET,
    }

    enum GameStates {
        ALIVE,
        DEAD,
    }

    pub struct Game {
        score: u32,
        board: [[u32; 4]; 4],
        state: GameStates,
    }

    impl Game {
        pub fn new() -> Self {
            Self {
                score: 0,
                board: [[0, 0, 0, 0], [0, 0, 2, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                state: GameStates::ALIVE,
            }
        }

        fn update_score(&mut self) {
            self.score = self
                .board
                .into_iter()
                .map(|row| row.into_iter().map(|cell| cell).sum::<u32>())
                .sum();
        }

        fn scan_diff(&self, next_board: &[[u32; 4]; 4]) -> bool {
            let mut i: usize = 0;
            let mut j: usize = 0;
            let mut is_diff: bool = false;

            // Loop over the array until a diff is found betweent the two arrays
            'row: loop {
                if i >= next_board.len() {
                    break;
                }

                'col: loop {
                    if j >= next_board[0].len() {
                        break 'col;
                    }

                    if next_board[i][j] != self.board[i][j] {
                        is_diff = true;
                        break 'row;
                    }

                    j += 1;
                    continue;
                }
                i += 1;
                continue;
            }

            is_diff
        }

        fn reset_board() -> [[u32; 4]; 4] {
            [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
        }

        fn fill_empty_pos(&mut self) {
            let mut rng = rand::thread_rng();

            let mut empty_positions: Vec<(usize, usize)> = vec![];

            for row in 0..self.board.len() {
                for col in 0..self.board[row].len() {
                    if self.board[row][col] == 0 {
                        empty_positions.push((row as usize, col as usize))
                    }
                }
            }

            let random_number: f64 = rng.gen();
            let selected: usize = (random_number * empty_positions.len() as f64).floor() as usize;
            let value: u32 = (random_number * 4.0).ceil() as u32;

            let (row, col) = empty_positions[selected];

            if value.abs_diff(4) > value.abs_diff(2) {
                self.board[row][col] = 2;
            } else {
                self.board[row][col] = 4;
            }
        }

        #[allow(unused)]
        fn print_board(&self) {
            let row_separator = "+---+---+---+---+";
            let col_separator = "|";

            for row in 0..self.board.len() {
                for col in 0..self.board[row].len() {
                    if col % 4 == 0 {
                        println!("{}", row_separator)
                    }
                    print!("{} {} ", col_separator, self.board[row][col]);
                    if col == 3 {
                        println!("{}", col_separator)
                    }
                    if col == 3 && row == 3 {
                        println!("{}", row_separator)
                    }
                }
            }
        }

        fn make_move(&self, next_move: Moves) -> [[u32; 4]; 4] {
            match next_move {
                Moves::LEFT => {
                    let size: usize = self.board.len();
                    let mut tmp_board = self.board.clone();
                    for row in 0..size {
                        let mut result_row: [u32; 4] = [0, 0, 0, 0];
                        let mut pointer: (usize, u32) = (0, 0);

                        for col in 0..size {
                            let (pos, val) = pointer;
                            if tmp_board[row][col] != 0 {
                                if tmp_board[row][col] == val {
                                    result_row[pos - 1] = val * 2;
                                    pointer = (pos, 0);
                                } else {
                                    result_row[pos] = tmp_board[row][col];
                                    pointer = (pos + 1, tmp_board[row][col]);
                                }
                            }
                        }

                        tmp_board[row] = result_row;
                    }

                    tmp_board
                }
                Moves::RIGHT => {
                    let size: usize = self.board.len();
                    let mut tmp_board = self.board.clone();
                    for row in 0..size {
                        let mut result_row: [u32; 4] = [0, 0, 0, 0];
                        let mut pointer: (usize, u32) = (size, 0);

                        for col in (0..size).rev() {
                            let (pos, val) = pointer;
                            if tmp_board[row][col] != 0 {
                                if tmp_board[row][col] == val {
                                    result_row[pos] = val * 2;
                                    pointer = (pos, 0);
                                } else {
                                    result_row[pos - 1] = tmp_board[row][col];
                                    pointer = (pos - 1, tmp_board[row][col]);
                                }
                            }
                        }

                        tmp_board[row] = result_row;
                    }

                    tmp_board
                }
                Moves::UP => {
                    let size: usize = self.board.len();
                    let mut tmp_board = self.board.clone();
                    for col in 0..size {
                        let mut result_col: [u32; 4] = [0, 0, 0, 0];
                        let mut pointer: (usize, u32) = (0, 0);

                        for row in 0..size {
                            let (pos, val) = pointer;

                            if tmp_board[row][col] != 0 {
                                if tmp_board[row][col] == val {
                                    result_col[pos - 1] = val * 2;
                                    pointer = (pos, 0);
                                } else {
                                    result_col[pos] = tmp_board[row][col];
                                    pointer = (pos + 1, tmp_board[row][col]);
                                }
                            }
                        }

                        for row in 0..size {
                            tmp_board[row][col] = result_col[row];
                        }
                    }

                    tmp_board
                }
                Moves::DOWN => {
                    let size: usize = self.board.len();
                    let mut tmp_board = self.board.clone();
                    for col in 0..size {
                        let mut result_col: [u32; 4] = [0, 0, 0, 0];
                        let mut pointer: (usize, u32) = (size, 0);

                        for row in (0..size).rev() {
                            let (pos, val) = pointer;

                            if tmp_board[row][col] != 0 {
                                if tmp_board[row][col] == val {
                                    result_col[pos] = val * 2;
                                    pointer = (pos, 0);
                                } else {
                                    result_col[pos - 1] = tmp_board[row][col];
                                    pointer = (pos - 1, tmp_board[row][col]);
                                }
                            }
                        }

                        for row in 0..size {
                            tmp_board[row][col] = result_col[row];
                        }
                    }

                    tmp_board
                }
                Moves::RESET => Game::reset_board(),
            }
        }

        fn update_game_state(&mut self) {
            let moves_to_try = [Moves::LEFT, Moves::RIGHT, Moves::UP, Moves::DOWN];
            let mut counter = 0;
            self.state = GameStates::DEAD;

            loop {
                if self.scan_diff(&self.make_move(moves_to_try[counter])) {
                    self.state = GameStates::ALIVE;
                    break;
                } else {
                    println!("No changes made with that move");
                }

                counter += 1;
            }
        }

        pub fn game_loop(&mut self, next_move: Moves) {
            match self.state {
                GameStates::ALIVE => {
                    let next_board: [[u32; 4]; 4] = self.make_move(next_move);

                    if self.scan_diff(&next_board) {
                        self.board = next_board.to_owned();
                        self.fill_empty_pos();
                    }

                    self.update_score();
                    self.print_board();
                    self.update_game_state();
                }
                GameStates::DEAD => println!("Game is dead, score is {}", self.score),
            }
        }
    }
}

use game_2048::{Game, Moves};

fn main() {
    let mut game = Game::new();
    println!("Welcome to the terminal version of 2048 made on Rust");

    loop {
        println!("Insert which move you want to perform next");
        print!("[1] Up\n[2] Down\n[3] Left\n[4] Right\n");

        let mut next_move = String::new();
        stdin()
            .read_line(&mut next_move)
            .expect("Did not input valid move");

        let next_move: u8 = next_move.trim().parse().expect("Did not input number");

        match next_move {
            1 => game.game_loop(Moves::UP),
            2 => game.game_loop(Moves::DOWN),
            3 => game.game_loop(Moves::LEFT),
            4 => game.game_loop(Moves::RIGHT),
            _ => println!("Incorrect number!"),
        }
    }
}
