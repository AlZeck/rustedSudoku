use std::collections::{HashSet, LinkedList};
use std::hash::Hash;
use std::process::exit;

#[derive(Copy, Eq)]
struct OptionVector {
    solved: usize,       // solved if self != 0
    options: [usize; 9], // if solved empty
}

impl PartialEq for OptionVector {
    fn eq(&self, other: &Self) -> bool {
        self.solved == other.solved
    }
}

impl Hash for OptionVector {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.solved.hash(state);
    }
}

impl Clone for OptionVector {
    fn clone(&self) -> Self {
        OptionVector {
            solved: self.solved,
            options: self.options.clone(),
        }
    }
}

impl OptionVector {
    fn new_solved(solved: usize) -> OptionVector {
        OptionVector {
            solved: solved,
            options: [0; 9],
        }
    }

    fn new_unsolved(options: [usize; 9]) -> OptionVector {
        OptionVector {
            solved: 0,
            options: options,
        }
    }

    fn remove_option(&mut self, index: usize) {
        if self.solved != 0 {
            return;
        }

        let mut i = index;

        while i < 8 && self.options[i + 1] != 0 {
            self.options[i] = self.options[i + 1];
            i += 1;
        }
        self.options[i] = 0;
    }

    fn solve(&mut self) -> Result<(), &str> {
        if self.solved == 0 && self.options[1] == 0 {
            self.solved = self.options[0];
            self.options[0] = 0;

            if self.solved == 0 {
                return Err("Unsolvable cell");
            }
        }
        Ok(())
    }

    fn is_solved(&self) -> bool {
        self.solved != 0
    }

    fn set_option(&mut self, index: usize) {
        if self.solved != 0 {
            return;
        }
        self.solved = self.options[index];
        self.options = [0; 9];
    }
}

pub struct Position {
    row: usize,
    col: usize,
}

#[derive(Copy, Clone, Eq, Hash)]
pub struct Sudoku {
    board: [[OptionVector; 9]; 9],
}

impl PartialEq for Sudoku {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                if !self.board[i][j].eq(&other.board[i][j]) {
                    return false;
                }
            }
        }
        true
    }
}

impl Sudoku {
    fn check_sub_area(&self, value: usize, pos: &Position) -> bool {
        if value == 0 {
            return true;
        }
        let mut set = [false; 9];
        let row = (pos.row / 3) * 3;
        let col = (pos.col / 3) * 3;
        for i in 0..3 {
            for j in 0..3 {
                let cell = &self.board[(row + i)][(col + j)];
                if cell.solved != 0 {
                    if set[cell.solved - 1] {
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
        if value == 0 {
            return true;
        }
        let mut set = [false; 9];
        for i in 0..9 {
            let cell = &self.board[pos.row][i];
            if cell.solved != 0 {
                if set[cell.solved - 1] {
                    return false;
                } else {
                    set[cell.solved - 1] = true;
                }
            }
        }
        // value is not in row
        !set[value - 1]
    }

    fn check_col(&self, value: usize, pos: &Position) -> bool {
        if value == 0 {
            return true;
        }
        let mut set = [false; 9];
        for i in 0..9 {
            let cell = &self.board[i][pos.col];
            if cell.solved != 0 {
                if set[cell.solved - 1] {
                    return false;
                } else {
                    set[cell.solved - 1] = true;
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
            if !self.check_row(0, &Position { row: i, col: 0 })
                || !self.check_col(0, &Position { row: 0, col: i })
            {
                return false;
            }
        }

        for i in 0..3 {
            for j in 0..3 {
                if !self.check_sub_area(
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

    fn validate_option(&self, value: usize, pos: Position) -> bool {
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
                        OptionVector::new_solved(c[i * 9 + j].to_digit(10).unwrap() as usize);
                } else {
                    sudoku.board[i][j] = OptionVector::new_unsolved([1, 2, 3, 4, 5, 6, 7, 8, 9]);
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

    pub fn solve(&mut self) -> Result<(), &str> {
        for i in 0..9 {
            for j in 0..9 {
                let mut cell = self.board[i][j];
                if cell.solved == 0 {
                    let mut k = 0;
                    while k < 9 && cell.options[k] != 0 {
                        if self.validate_option(cell.options[k], Position { row: i, col: j }) {
                            k += 1;
                        } else {
                            cell.remove_option(k);
                        }
                    }
                    if cell.solve().is_err() {
                        return Err("Could not solve");
                    }
                    self.board[i][j] = cell;
                }
            }
        }
        Ok(())
    }

    pub fn to_string(&self) -> String {
        let mut string = String::new();
        for i in 0..9 {
            for j in 0..9 {
                let cell = &self.board[i][j];
                if cell.solved != 0 {
                    string.push_str(cell.solved.to_string().as_str());
                } else {
                    string.push('.');
                }
            }
        }
        string
    }
}

pub fn master_solve(sudoku: &Sudoku) -> HashSet<Sudoku> {
    let mut options_stack: LinkedList<Sudoku> = LinkedList::new();
    let mut solved: HashSet<Sudoku> = HashSet::new();
    options_stack.push_back(sudoku.clone());

    while options_stack.len() > 0 {
        let mut sudoku = options_stack.pop_front().unwrap();
        if sudoku.solve().is_ok() {
            if sudoku.is_solved() {
                if !solved.contains(&sudoku) {
                    solved.insert(sudoku);
                    sudoku.print();
                    println!();
                }
            } else {
                let mut i = 0;
                let mut j = 0;
                while i < 9 {
                    if !(sudoku.board[i][j].is_solved()) {
                        let mut k = 0;
                        while k < 9 && sudoku.board[i][j].options[k] != 0 {
                            let mut new_sudoku = sudoku.clone();
                            new_sudoku.board[i][j].set_option(k);

                            if !(options_stack.contains(&new_sudoku)) {
                                options_stack.push_back(new_sudoku);
                            } else {
                                println!("already in stack");
                            }

                            k += 1;
                        }
                        break;
                    } else {
                        j += 1;
                        if j == 9 {
                            i += 1;
                            j = 0;
                        }
                    }
                }
            }
        }
    }
    solved
}
