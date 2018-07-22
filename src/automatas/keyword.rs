pub fn is_keyword(token: &String) -> bool {
    match get_keywords()
        .into_iter()
        .filter(|&keyword| keyword == token)
        .next()
    {
        Some(_) => true,
        None => false,
    }
}

fn get_keywords() -> [String; 16] {
    [
        String::from("int"),
        String::from("float"),
        String::from("char"),
        String::from("bool"),
        String::from("str"),
        String::from("let"),
        String::from("lem"),
        String::from("if"),
        String::from("else"),
        String::from("while"),
        String::from("for"),
        String::from("to"),
        String::from("in"),
        String::from("fn"),
        String::from("ret"),
        String::from("call"),
    ]
}
