pub fn is_limiter(token: &String) -> bool {
    match get_limiters()
        .into_iter()
        .filter(|&limiter| limiter == token)
        .next()
    {
        Some(_) => true,
        None => false,
    }
}

fn get_limiters() -> [String; 8] {
    [
        String::from(";"),
        String::from("("),
        String::from(")"),
        String::from("{"),
        String::from("}"),
        String::from(","),
        String::from("."),
        String::from(":"),
    ]
}
