use automatas::{Category, Lexem};

pub fn syntax_analysis(symbols: &Vec<Lexem>) {
    let mut input: Vec<String> = Vec::new();
    for symbol in symbols {
        input.push(match symbol.category {
            Category::ID => String::from("identifier"),
            Category::VALUE => String::from("value"),
            Category::OPERATOR => String::from("operator"),
            Category::KEYWORD => match symbol.token.as_ref() {
                "int" | "float" | "char" | "str" | "bool" => String::from("data_type"),
                _ => symbol.token.clone(),
            },
            _ => symbol.token.clone(),
        })
    }
    input.push(String::from("$"));

    for stuff in input {
        println!("{}", stuff);
    }
}
