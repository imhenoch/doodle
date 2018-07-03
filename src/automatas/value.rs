use automatas::{boolean, char_automata, float_automata, int_automata, string_automata, DataType};

pub fn is_value(token: &mut String) -> (bool, DataType, i8) {
    if boolean::is_boolean(token.clone()) {
        (true, DataType::BOOL, 1)
    } else if char_automata::is_char(&mut token.clone()) {
        (true, DataType::CHAR, 1)
    } else if string_automata::is_string(&mut token.clone()) {
        (true, DataType::STR, 0)
    } else if int_automata::is_int(&mut token.clone()) {
        (true, DataType::INT, 8)
    } else if float_automata::is_float(&mut token.clone()) {
        (true, DataType::FLOAT, 8)
    } else {
        (false, DataType::NONE, -1)
    }
}
