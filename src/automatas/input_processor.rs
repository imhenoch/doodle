use automatas::{id_automata, keyword, limiter, operator, scope, value, Category, Lexem};
use input_formatter::InputSlice;

pub fn get_symbols(input: Vec<InputSlice>) -> Vec<Lexem> {
    let mut symbols: Vec<Lexem> = Vec::new();
    for slice in input {
        let mut lexem = Lexem::new(slice.token, slice.column, slice.row);
        find_category(&mut lexem);
        symbols.push(lexem);
    }
    // scope::set_scope(&mut symbols);
    symbols
}

fn find_category(lexem: &mut Lexem) {
    let token = lexem.token.clone();
    lexem.set_category(if keyword::is_keyword(&token.clone()) {
        Category::KEYWORD
    } else if id_automata::is_id(&mut token.clone()) {
        Category::ID
    } else if operator::is_operator(&token.clone()) {
        Category::OPERATOR
    } else if limiter::is_limiter(&token.clone()) {
        Category::LIMITER
    } else {
        Category::NONE
    });

    let (is_value, data_type) = value::is_value(&mut token.clone());
    if is_value {
        lexem.set_data_type(data_type);
        lexem.set_category(Category::VALUE);
    }
}
