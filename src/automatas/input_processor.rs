use automatas::{Category, Lexem};
use lexer::InputSlice;
use std::collections::HashMap;

pub fn get_symbol_table(input: Vec<InputSlice>) -> Result<HashMap<u32, Lexem>, String> {
    let mut symbol_table: HashMap<u32, Lexem> = HashMap::new();
    for slice in input {
        println!("{}", slice);
        let mut lexem = Lexem::new(slice.token, slice.column, slice.row);
        find_category(&mut lexem);
        match lexem.category {
            Category::NONE => {
                return Err(String::from(format!(
                    "Unrecognizable token on column {}, row {}",
                    lexem.column, lexem.row,
                )));
            }
            _ => {
                // symbol_table.insert(1, lexem);
            }
        }
    }
    Ok(symbol_table)
}

fn find_category(lexem: &mut Lexem) {}
