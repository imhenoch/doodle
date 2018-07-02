use automatas::{boolean, string_automata};

pub fn is_value(token: &mut String) -> bool {
    boolean::is_boolean(token.clone()) || string_automata::is_string(&mut token.clone())
}
