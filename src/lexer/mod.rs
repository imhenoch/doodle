pub mod string_processor;

pub const INT_TYPE: u8 = 1;
pub const FLOAT_TYPE: u8 = 2;
pub const CHAR_TYPE: u8 = 3;
pub const BOOL_TYPE: u8 = 4;
pub const STRING_TYPE: u8 = 5;

pub struct Symbol {
    pub token: String,
    pub data_type: Option<u8>,
    pub category: Option<u8>,
    pub context: Option<String>,
}

impl Symbol {
    pub fn new(token: String) -> Symbol {
        Symbol {
            token: token,
            data_type: None,
            category: None,
            context: None,
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
