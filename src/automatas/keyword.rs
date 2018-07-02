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

fn get_keywords() -> [String; 14] {
    [
        String::from("int"),
        String::from("float"),
        String::from("char"),
        String::from("bool"),
        String::from("str"),
        String::from("let"),
        String::from("mut"),
        String::from("if"),
        String::from("else"),
        String::from("while"),
        String::from("for"),
        String::from("to"),
        String::from("fn"),
        String::from("ret"),
    ]
}
