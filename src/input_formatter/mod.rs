use std::fmt;

pub mod string_processor;

pub const LIMITERS: [char; 8] = [';', '\n', ' ', '(', ')', '{', '}', ','];

pub struct InputSlice {
    pub token: String,
    pub column: u32,
    pub row: u32,
}

impl InputSlice {
    pub fn new(token: String, column: u32, row: u32) -> InputSlice {
        InputSlice {
            token: token,
            column: column,
            row: row,
        }
    }
}

impl fmt::Display for InputSlice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{token: '{}', column: {}, row: {}}}",
            self.token, self.column, self.row
        )
    }
}
