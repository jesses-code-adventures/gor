#[derive(Debug, PartialEq)]
pub struct Position {
    line: u32,
    column_start: u32,
    column_end: u32,
}

impl Position {
    pub fn new(line: u32, column_start: u32, column_end: u32) -> Position {
        Position {
            line,
            column_start,
            column_end,
        }
    }
}
