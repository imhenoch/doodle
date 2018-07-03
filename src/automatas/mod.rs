use std::fmt;

pub mod boolean;
pub mod char_automata;
pub mod float_automata;
pub mod id_automata;
pub mod input_processor;
pub mod int_automata;
pub mod keyword;
pub mod limiter;
pub mod operator;
pub mod scope;
pub mod string_automata;
pub mod value;

pub enum DataType {
    NONE,
    INT,
    FLOAT,
    BOOL,
    CHAR,
    STR,
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DataType::NONE => "NONE",
                DataType::INT => "INT",
                DataType::FLOAT => "FLOAT",
                DataType::BOOL => "BOOL",
                DataType::CHAR => "CHAR",
                DataType::STR => "STR",
            }
        )
    }
}

pub enum Category {
    NONE,
    KEYWORD,
    ID,
    OPERATOR,
    VALUE,
    LIMITER,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Category::NONE => "NONE",
                Category::KEYWORD => "KEYWORD",
                Category::ID => "ID",
                Category::OPERATOR => "OPERATOR",
                Category::VALUE => "VALUE",
                Category::LIMITER => "LIMITER",
            }
        )
    }
}

pub struct Lexem {
    pub token: String,
    pub data_type: DataType,
    pub size: i8,
    pub category: Category,
    pub scope: String,
    pub column: u32,
    pub row: u32,
}

impl Lexem {
    fn new(token: String, column: u32, row: u32) -> Lexem {
        Lexem {
            token: token,
            data_type: DataType::NONE,
            category: Category::NONE,
            size: -1,
            scope: String::from(""),
            column: column,
            row: row,
        }
    }

    fn set_data_type(&mut self, data_type: DataType) {
        self.data_type = data_type;
    }

    fn set_size(&mut self, size: i8) {
        self.size = size;
    }

    fn set_category(&mut self, category: Category) {
        self.category = category;
    }

    fn set_scope(&mut self, scope: String) {
        self.scope = scope;
    }
}

impl fmt::Display for Lexem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "token: {}, category: {}, type: {}, column: {}, row: {}, scope: {}",
            self.token, self.category, self.data_type, self.column, self.row, self.scope
        )
    }
}
