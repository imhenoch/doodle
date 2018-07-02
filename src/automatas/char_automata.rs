pub fn is_char(token: &mut String) -> bool {
    q0(token)
}

fn q0(token: &mut String) -> bool {
    match token.remove(0) {
        '\'' => q1(token),
        _ => false,
    }
}

fn q1(token: &mut String) -> bool {
    if token.is_empty() {
        false
    } else {
        token.remove(0);
        q2(token)
    }
}

fn q2(token: &mut String) -> bool {
    if token.is_empty() {
        false
    } else {
        match token.remove(0) {
            '\'' => q3(token),
            _ => false,
        }
    }
}

fn q3(token: &mut String) -> bool {
    token.is_empty()
}
