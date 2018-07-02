use automatas::boolean;

pub fn is_value(token: &mut String) -> bool {
    boolean::is_boolean(token.clone())
}
