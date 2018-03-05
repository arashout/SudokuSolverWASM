use std::fmt;

pub const BOARD_LENGTH: usize = 9;
const BLOCK_LENGTH: usize = 3;

#[derive(Copy, Clone)]
pub struct SudokuBoard {
    pub numbers: [usize; (BOARD_LENGTH * BOARD_LENGTH)],
}

impl fmt::Display for SudokuBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..BOARD_LENGTH {
            for col in 0..BOARD_LENGTH {
                write!(f, "{} ", self.get_number(&Position { row: row, col: col }))
                    .expect("Could not add number!");
            }
            write!(f, "\n").expect("Could not add new line!");
        }
        return write!(f, "");
    }
}

#[derive(PartialEq)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

fn get_square(p: &Position) -> Position {
    let rb = p.row / BLOCK_LENGTH;
    let cb = p.col / BLOCK_LENGTH;
    return Position { row: rb, col: cb };
}

fn get_index(p: &Position) -> usize {
    return p.col + p.row * BOARD_LENGTH;
}

fn get_position(i: usize) -> Position {
    return Position {
        row: i / BOARD_LENGTH,
        col: i % BOARD_LENGTH,
    };
}

impl SudokuBoard {
    pub fn new(line: String) -> SudokuBoard {
        let mut sb = SudokuBoard {
            numbers: [0; (BOARD_LENGTH * BOARD_LENGTH)],
        };
        for (i, c) in line.chars().enumerate() {
            if c != '\n' {
                sb.numbers[i] = c.to_digit(10).expect("Not unsigned integer") as usize;
            }
        }
        return sb;
    }

    pub fn first_empty_position(&self) -> Option<Position> {
        for i in 0..BOARD_LENGTH * BOARD_LENGTH {
            if self.numbers[i as usize] == 0 {
                return Some(get_position(i));
            }
        }
        return None;
    }

    pub fn set_num(&self, n: usize, p: &Position) -> SudokuBoard {
        let mut new_board = self.clone();
        let i = get_index(p);
        new_board.numbers[i as usize] = n;
        return new_board;
    }

    pub fn get_number(&self, ref p: &Position) -> usize {
        let i = get_index(&p);
        return self.numbers[i];
    }

    pub fn is_in_row(&self, n: usize, p: &Position) -> bool {
        for col in 0..BOARD_LENGTH {
            let p_to_check = Position {
                row: p.row,
                col: col,
            };

            if p == &p_to_check {
                continue;
            }

            let n_to_check = self.get_number(&p_to_check);
            if n_to_check == n {
                return true;
            }
        }
        return false;
    }

    pub fn is_in_column(&self, n: usize, p: &Position) -> bool {
        for row in 0..BOARD_LENGTH {
            // TODO: Skip tile itself

            let p_to_check = Position {
                row: row,
                col: p.col,
            };

            if p == &p_to_check {
                continue;
            }

            let n_to_check = self.get_number(&p_to_check);

            if n_to_check == n {
                return true;
            }
        }
        return false;
    }

    pub fn is_in_square(&self, n: usize, p: &Position) -> bool {
        let square = get_square(&p);
        let left_corner = BOARD_LENGTH * BLOCK_LENGTH * square.row + BLOCK_LENGTH * square.col;

        let current_index = get_index(&p);
        for i in 0..BLOCK_LENGTH {
            for j in 0..BLOCK_LENGTH {
                let index = left_corner + i * BOARD_LENGTH + j;

                if index == current_index {
                    continue;
                }

                if self.numbers[index] == n {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn is_valid_move(&self, n: usize, p: &Position) -> bool {
        return !self.is_in_square(n, &p) && !self.is_in_row(n, &p) && !self.is_in_column(n, &p);
    }

    pub fn is_valid_sudoku(&self) -> bool {
        for r in 0..BOARD_LENGTH {
            for c in 0..BOARD_LENGTH {
                let p = Position { row: r, col: c };
                let n = self.get_number(&p);
                if !self.is_valid_move(n, &p) {
                    return false;
                }
            }
        }
        return true;
    }
}

mod tests {
    use board;

    fn get_board() -> board::SudokuBoard {
        let s = "003020600900305001001806400008102900700000008006708200002609500800203009005010300";
        return board::SudokuBoard::new(s.to_string());
    }

    #[test]
    fn test_is_in_row() {
        let b = get_board();
        let p = board::Position { row: 1, col: 0 };

        // Should ignore current position
        assert!(!b.is_in_row(9, &p));

        assert!(b.is_in_row(1, &p));
        assert!(b.is_in_row(3, &p));

        assert!(!b.is_in_row(4, &p));
    }

    #[test]
    fn test_is_in_square() {
        let b = get_board();
        let p = board::Position { row: 3, col: 3 };

        // Should ignore current position
        assert!(!b.is_in_square(1, &p));

        assert!(b.is_in_square(2, &p));
        assert!(b.is_in_square(7, &p));

        assert!(!b.is_in_square(9, &p));
    }

    #[test]
    fn test_is_in_column() {
        let b = get_board();
        let p = board::Position { row: 2, col: 6 };

        // Should ignore current position
        assert!(!b.is_in_column(4, &p));

        assert!(b.is_in_column(6, &p));
        assert!(b.is_in_column(9, &p));

        assert!(!b.is_in_column(1, &p));
    }

    #[test]
    fn test_is_valid_move() {
        let b = get_board();
        let p = board::Position { row: 0, col: 0 };

        assert!(b.is_valid_move(4, &p));

        // Square violation
        assert!(!b.is_valid_move(1, &p));
        // Column violation
        assert!(!b.is_valid_move(7, &p));
        // Row violation
        assert!(!b.is_valid_move(2, &p));
    }

    #[test]
    fn test_is_valid_sudoku() {
        let unfilled_board = get_board();
        assert!(!unfilled_board.is_valid_sudoku());

        let s = "845632179732918654196745328683574912457291836219863547361429785574186293928357461";
        let valid_board = board::SudokuBoard::new(s.to_string());
        assert!(valid_board.is_valid_sudoku());
    }

    #[test]
    fn test_set_num() {
        let b = get_board();
        let n = 2;

        let p = board::Position { row: 0, col: 1 };
        let new_b = b.set_num(n, &p);

        assert!(new_b.get_number(&p) == n);
    }
}
