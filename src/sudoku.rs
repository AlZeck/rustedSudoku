#[derive(Clone, Copy)]
struct OptionVector {
    solved: usize,       // solved if self != 0
    options: [usize; 9], // if solved empty
}

struct Position {
    row: usize,
    col: usize,
}

pub struct Sudoku {
    board: [[OptionVector; 9]; 9],
}

struct SolutionsStack {
    solutions: Vec<Sudoku>,
}

impl Sudoku {
    fn check_sub_area(&self, value: usize, pos: &Position) -> bool {
        let mut set = [false; 9];
        let row = (pos.row / 3) * 3;
        let col = (pos.col / 3) * 3;
        for i in 0..3 {
            for j in 0..3 {
                let cell = &self.board[(row + i)][(col + j)];
                if cell.solved != 0 {
                    if !set[cell.solved - 1] {
                        return false;
                    } else {
                        set[cell.solved - 1] = true;
                    }
                }
            }
        }
        // value is not in sub area
        !set[value - 1]
    }

    fn check_row(&self, value: usize, pos: &Position) -> bool {
        let mut set = [false; 9];
        for i in 0..9 {
            let cell = &self.board[pos.row][i];
            if cell.solved != 0 {
                if !set[&cell.solved - 1] {
                    return false;
                } else {
                    set[&cell.solved - 1] = true;
                }
            }
        }
        // value is not in row
        !set[value - 1]
    }

    fn check_col(&self, value: usize, pos: &Position) -> bool {
        let mut set = [false; 9];
        for i in 0..9 {
            let cell = &self.board[i][pos.col];
            if cell.solved != 0 {
                if !set[&cell.solved - 1] {
                    return false;
                } else {
                    set[&cell.solved - 1] = true;
                }
            }
        }
        // value is not in col
        !set[value - 1]
    }

    pub fn is_solved(&self) -> bool {
        for row in &self.board {
            for cell in row {
                if cell.solved == 0 {
                    return false;
                }
            }
        }

        for i in 0..9 {
            if self.check_row(0, &Position { row: i, col: 0 })
                || self.check_col(0, &Position { row: 0, col: i })
            {
                return false;
            }
        }

        for i in 0..3 {
            for j in 0..3 {
                if self.check_sub_area(
                    0,
                    &Position {
                        row: i * 3,
                        col: j * 3,
                    },
                ) {
                    return false;
                }
            }
        }

        true
    }

    pub fn validate_option(&self, value: usize, pos: Position) -> bool {
        if self.board[pos.row][pos.col].solved != 0 {
            return false;
        }

        self.check_row(value, &pos)
            && self.check_col(value, &pos)
            && self.check_sub_area(value, &pos)
    }

    pub fn create_from_string(string: &str) -> Sudoku {
        let mut sudoku = Sudoku {
            board: [[OptionVector {
                solved: 0,
                options: [0; 9],
            }; 9]; 9],
        };
        let c = string.chars().collect::<Vec<char>>();
        for i in 0..9 {
            for j in 0..9 {
                if c[i * 9 + j] != '.' {
                    sudoku.board[i][j] =
                        generate_vector(c[i * 9 + j].to_digit(10).unwrap() as usize);
                } else {
                    sudoku.board[i][j] = generate_vector(0)
                }
            }
        }
        sudoku
    }

    pub fn print(&self) {
        for i in 00..self.board.len() {
            let row = &self.board[i];
            print!("\t");
            for j in 0..row.len() {
                let cell = &row[j];
                if cell.solved != 0 {
                    print!(" {} \x1B[0m", cell.solved);
                } else {
                    print!("   \x1B[0m");
                }
                if j < 8 {
                    if (j + 1) % 3 == 0 {
                        print!("\x1B[1m║\x1B[0m");
                    } else {
                        print!("\x1B[2;38;5;250m║\x1B[0m");
                    }
                }
            }
            if i < 8 {
                print!("\n\t");
                for char in 0..35 {
                    let char_to_print = if char != 0 && (char + 1) % 4 == 0 {
                        if (i + 1) % 3 == 0 || char == 11 || char == 23 {
                            "\x1B[1m╬\x1B[0m"
                        } else {
                            "\x1B[2;38;5;250m╬\x1B[0m"
                        }
                    } else {
                        if (i + 1) % 3 == 0 {
                            "\x1B[1m═\x1B[0m"
                        } else {
                            "\x1B[2;38;5;250m═\x1B[0m"
                        }
                    };
                    print!("{}", char_to_print);
                }
            }
            println!();
        }
    }
}

fn generate_vector(solved: usize) -> OptionVector {
    let options = if solved == 0 {
        [1, 2, 3, 4, 5, 6, 7, 8, 9]
    } else {
        [0; 9]
    };
    OptionVector {
        solved: solved,
        options: options,
    }
}
