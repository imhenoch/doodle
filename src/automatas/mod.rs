use std::fmt;

pub mod input_processor;

pub enum DataType {
    NONE,
    INT,
    FLOAT,
    BOOL,
    CHAR,
    STR,
}

pub enum Category {
    NONE,
    KEY_WORD,
    ID,
    OPERATION,
    VALUE,
}

pub struct Lexem {
    pub token: String,
    pub data_type: DataType,
    pub category: Category,
    pub context: Option<String>,
    pub column: u32,
    pub row: u32,
}

impl Lexem {
    fn new(token: String, column: u32, row: u32) -> Lexem {
        Lexem {
            token: token,
            data_type: DataType::NONE,
            category: Category::NONE,
            context: None,
            column: column,
            row: row,
        }
    }

    fn set_data_type(&mut self, data_type: DataType) {
        self.data_type = data_type;
    }

    fn set_category(&mut self, category: Category) {
        self.category = category;
    }

    fn set_context(&mut self, context: String) {
        self.context = Some(context);
    }
}
