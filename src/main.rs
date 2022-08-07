mod sudoku;

fn main() {
    let sudoku = sudoku::Sudoku::create_from_string(
        "6....894.9....61...7..4....2..61..........2...89..2.......6...5.......3.8....16..",
    );

    sudoku.print()
}
