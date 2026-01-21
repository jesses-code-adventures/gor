#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Position {
    line: usize,
    column_start: usize,
    column_end: usize,
}

impl Position {
    pub fn new(line: usize, column_start: usize, column_end: usize) -> Position {
        Position {
            line,
            column_start,
            column_end,
        }
    }
}
