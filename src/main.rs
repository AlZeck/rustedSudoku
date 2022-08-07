mod sudoku;

fn main() {
    let mut sudoku = sudoku::Sudoku::create_from_string(
        "517483962.8.6.27..4.........3.75..28..8.....5.4.12..962.........7.2.45...568....1",
    );

    sudoku.print();
    println!();
    sudoku.solve();
    sudoku.print();
    println!("{}", sudoku.is_solved());
}
