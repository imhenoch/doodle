use lexer::{Lexem, LIMITERS};

pub fn transform_string_to_collection(input: String) {
    let mut lexems: Vec<Lexem> = Vec::new();
    let mut x0 = 0;
    let mut x1 = 0;
    let mut column = 0;
    let mut row = 0;
    let len = input.len();
    let mut chars = input.chars();
    let mut skip_line = false;

    while x1 < len {
        let c = chars.next().unwrap();
        if is_limiter(&c) {
            if x0 != x1 {
                let s = input.get(x0..x1).unwrap();
                if s == "//" {
                    skip_line = true;
                } else if !skip_line {
                    lexems.push(Lexem::new(s.to_string(), column - s.len() as u32, row));
                }
                x0 = x1 + 1;
            } else {
                x0 += 1;
            }
            if c == ';' || c == '(' || c == ')' || c == '{' || c == '}' || c == ',' {
                lexems.push(Lexem::new(c.to_string(), column as u32, row));
            }
            if c == '\n' {
                skip_line = false;
                row += 1;
                column = 0;
            }
        }
        column += 1;
        x1 += 1;
    }
    if x0 != x1 {
        let s = input.get(x0..x1).unwrap();
        lexems.push(Lexem::new(s.to_string(), column - s.len() as u32, row));
    }
    for lexem in lexems {
        println!("lexem: {}", lexem);
    }
}

fn is_limiter(limiter: &char) -> bool {
    for l in LIMITERS.iter() {
        if limiter == l {
            return true;
        }
    }
    false
}
