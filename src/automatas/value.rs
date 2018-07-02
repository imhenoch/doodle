use automatas::{boolean, char_automata, float_automata, int_automata, string_automata, DataType};

pub fn is_value(token: &mut String) -> (bool, DataType) {
    if boolean::is_boolean(token.clone()) {
        (true, DataType::BOOL)
    } else if string_automata::is_string(&mut token.clone()) {
        (true, DataType::STR)
    } else if char_automata::is_char(&mut token.clone()) {
        (true, DataType::CHAR)
    } else if int_automata::is_int(&mut token.clone()) {
        (true, DataType::INT)
    } else if float_automata::is_float(&mut token.clone()) {
        (true, DataType::FLOAT)
    } else {
        (false, DataType::NONE)
    }
}
