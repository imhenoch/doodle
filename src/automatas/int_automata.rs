pub fn is_int(token: &mut String) -> bool {
    q0(token)
}

fn q0(token: &mut String) -> bool {
    if token.is_empty() {
        true
    } else {
        match token.remove(0) as u32 {
            // 0-9
            48...57 => q0(token),
            _ => false,
        }
    }
}
