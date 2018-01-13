use board;

struct SolveResult {
    pub sudoku_board: board::SudokuBoard,
    pub is_solved: bool,
}

pub fn solve(b: &board::SudokuBoard) -> board::SudokuBoard {
    return back_track(b).sudoku_board;
}

fn back_track(b: &board::SudokuBoard) -> SolveResult {
    let result = b.first_empty_position();

    match result {
        Some(p) => {
            for possible_n in 1..(board::BOARD_LENGTH + 1) {
                if b.is_valid_move(possible_n, &p) {
                    let new_board = b.set_num(possible_n, &p);
                    let result = back_track(&new_board);
                    if result.is_solved {
                        return result;
                    }
                }
            }
            return SolveResult {
                sudoku_board: *b,
                is_solved: false,
            };
        }
        None => {
            if b.is_valid_sudoku() {
                return SolveResult {
                    sudoku_board: b.clone(),
                    is_solved: true,
                };
            }
            panic!("Sudoku Board is unsolvable!");
        }
    }
}
