struct OptionVector {
    solved: u8,       // solved if self != 0
    options: [u8; 9], // if solved empty
}

struct Position {
    row: u8,
    col: u8,
}

struct Sudoku {
    board: [[OptionVector; 9]; 9],
}

struct SolutionsStack {
    solutions: Vec<Sudoku>,
}

impl Sudoku {
    fn is_solved(&self) -> bool {
        for row in &self.board {
            for cell in row {
                if cell.solved == 0 {
                    return false;
                }
            }
        }
        return true;
    }

   
}

fn generate_vector(solved: u8, options: [u8; 9]) -> OptionVector {
    OptionVector {
        solved: solved,
        options: options,
    }
}
