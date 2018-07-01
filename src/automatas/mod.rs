pub mod input_processor;

pub enum DataType {
    INT,
    FLOAT,
    BOOL,
    CHAR,
    STR,
}

pub enum Category {
    KEY_WORD,
    ID,
    OPERATION,
    VALUE,
}

pub struct Lexem {
    pub token: String,
    pub data_type: Option<DataType>,
    pub category: Option<Category>,
    pub context: Option<String>,
    pub column: u32,
    pub row: u32,
}

impl Lexem {
    fn new(token: String, column: u32, row: u32) -> Lexem {
        Lexem {
            token: token,
            data_type: None,
            category: None,
            context: None,
            column: column,
            row: row,
        }
    }

    fn set_data_type(&mut self, data_type: DataType) {
        self.data_type = Some(data_type);
    }

    fn set_category(&mut self, category: Category) {
        self.category = Some(category);
    }

    fn set_context(&mut self, context: String) {
        self.context = Some(context);
    }
}
