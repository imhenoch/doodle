mod hash;

use automatas::{Category, Lexem};
use std::collections::HashMap;

pub fn get_symbol_table(symbols: &Vec<Lexem>) -> (HashMap<u64, &Lexem>, Vec<String>) {
    let mut hashmap: HashMap<u64, &Lexem> = HashMap::new();
    let mut errors = Vec::new();

    for symbol in symbols {
        match symbol.category {
            Category::NONE => {
                errors.push(format!(
                    "Unrecognized token \"{}\" at line {}, {}",
                    symbol.token, symbol.row, symbol.column,
                ));
            }
            _ => {
                let hash = hash::hash(&format!("{}{}", symbol.token, symbol.scope));
                if !hashmap.contains_key(&hash) {
                    hashmap.insert(hash, symbol);
                }
            }
        }
    }

    (hashmap, errors)
}
