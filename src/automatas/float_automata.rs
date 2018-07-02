pub fn is_float(token: &mut String) -> bool {
    q0(token)
}

fn q0(token: &mut String) -> bool {
    if token.is_empty() {
        true
    } else {
        match token.remove(0) as u32 {
            // 0-9
            48...57 => q0(token),
            // .
            46 => q1(token),
            _ => false,
        }
    }
}

fn q1(token: &mut String) -> bool {
    if token.is_empty() {
        false
    } else {
        match token.remove(0) as u32 {
            // 0-9
            48...57 => q2(token),
            _ => false,
        }
    }
}

fn q2(token: &mut String) -> bool {
    if token.is_empty() {
        true
    } else {
        match token.remove(0) as u32 {
            // 0-9
            48...57 => q2(token),
            _ => false,
        }
    }
}
