pub fn is_keyword(token: &String) -> bool {
    for keyword in get_keywords().into_iter() {
        if token == keyword {
            return true;
        }
    }
    false
}

fn get_keywords() -> [String; 12] {
    [
        String::from("int"),
        String::from("float"),
        String::from("char"),
        String::from("bool"),
        String::from("str"),
        String::from("let"),
        String::from("mut"),
        String::from("if"),
        String::from("while"),
        String::from("for"),
        String::from("fun"),
        String::from("ret"),
    ]
}
