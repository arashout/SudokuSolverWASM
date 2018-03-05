use std::time::Instant;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod board;
mod solver;

fn main() {    
    let f = File::open("resources/puzzles.txt").expect("File not found!");

    let reader = BufReader::new(&f);
    for (num, line) in reader.lines().enumerate() {
        let line = line.expect("Corrupt line!");
        let sb = board::SudokuBoard::new(line);

        let now = Instant::now();
        let solved_board = solver::solve(&sb);
        // TODO: Use more precise timing than seconds
        println!(
            "Solved puzzle number: {} in {} seconds",
            num+1,
            now.elapsed().as_secs()
        );
        println!("{}", solved_board);
    }
}
