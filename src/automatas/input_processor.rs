use automatas::{id_automata, keyword, operator, value, Category, Lexem};
use input_formatter::InputSlice;
use std::collections::HashMap;

pub fn get_symbol_table(input: Vec<InputSlice>) -> Result<HashMap<u32, Lexem>, String> {
    let mut symbol_table: HashMap<u32, Lexem> = HashMap::new();
    let mut a = 0;
    for slice in input {
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
                symbol_table.insert(a, lexem);
                a += 1;
            }
        }
    }
    Ok(symbol_table)
}

fn find_category(lexem: &mut Lexem) {
    let token = lexem.token.clone();
    lexem.set_category(if keyword::is_keyword(&token.clone()) {
        Category::KEYWORD
    } else if value::is_value(&mut token.clone()) {
        Category::VALUE
    } else if id_automata::is_id(&mut token.clone()) {
        Category::ID
    } else if operator::is_operator(&token.clone()) {
        Category::OPERATOR
    } else {
        Category::NONE
    });
}
