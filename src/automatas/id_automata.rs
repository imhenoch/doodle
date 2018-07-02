pub fn is_id(token: &mut String) -> bool {
    q0(token)
}

fn q0(token: &mut String) -> bool {
    match token.remove(0) as u32 {
        // A-Z    _      a-z
        65...90 | 95 | 97...122 => q1(token),
        _ => false,
    }
}

fn q1(token: &mut String) -> bool {
    if token.is_empty() {
        true
    } else {
        match token.remove(0) as u32 {
            // 0-9      A-Z     _      a-z
            48...57 | 65...90 | 95 | 97...122 => q1(token),
            _ => false,
        }
    }
}
