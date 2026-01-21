#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Position {
    pub line: usize,
    pub column_start: usize,
    pub column_end: usize,
}

impl Position {
    pub fn new(line: usize, column_start: usize, column_end: usize) -> Position {
        Position {
            line,
            column_start,
            column_end,
        }
    }

    pub fn new_single_position(line: usize, column: usize) -> Position {
        Position {
            line,
            column_start: column,
            column_end: column,
        }
    }
}
