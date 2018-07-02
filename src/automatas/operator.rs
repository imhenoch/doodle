pub fn is_operator(token: &String) -> bool {
    match get_operators()
        .into_iter()
        .filter(|&operator| operator == token)
        .next()
    {
        Some(_) => true,
        None => false,
    }
}

fn get_operators() -> [String; 16] {
    [
        String::from("+"),
        String::from("-"),
        String::from("*"),
        String::from("/"),
        String::from("%"),
        String::from("^"),
        String::from("="),
        String::from("&&"),
        String::from("||"),
        String::from("=="),
        String::from("!"),
        String::from("!="),
        String::from(">"),
        String::from("<"),
        String::from(">="),
        String::from("<="),
    ]
}
