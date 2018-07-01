use std::fmt;

pub mod string_processor;

pub const INT_TYPE: u8 = 1;
pub const FLOAT_TYPE: u8 = 2;
pub const CHAR_TYPE: u8 = 3;
pub const BOOL_TYPE: u8 = 4;
pub const STRING_TYPE: u8 = 5;

pub const LIMITERS: [char; 8] = [';', '\n', ' ', '(', ')', '{', '}', ','];

pub struct Lexem {
    pub token: String,
    pub data_type: Option<u8>,
    pub category: Option<u8>,
    pub context: Option<String>,
    pub column: u32,
    pub row: u32,
}

impl Lexem {
    pub fn new(token: String, column: u32, row: u32) -> Lexem {
        Lexem {
            token: token,
            data_type: None,
            category: None,
            context: None,
            column: column,
            row: row,
        }
    }

    pub fn set_data_type(&mut self, data_type: u8) {
        self.data_type = Some(data_type);
    }

    pub fn set_category(&mut self, category: u8) {
        self.category = Some(category);
    }

    pub fn set_context(&mut self, context: String) {
        self.context = Some(context);
    }
}

impl fmt::Display for Lexem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{token: '{}', column: {}, row: {}}}",
            self.token, self.column, self.row
        )
    }
}
