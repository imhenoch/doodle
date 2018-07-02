use input_formatter::{InputSlice, LIMITERS};

pub fn transform_string_to_collection(input: String) -> Vec<InputSlice> {
    let mut lexems: Vec<InputSlice> = Vec::new();
    let mut x0 = 0;
    let mut x1 = 0;
    let mut column = 0;
    let mut row = 0;
    let len = input.len();
    let mut chars = input.chars();
    let mut skip_line = false;
    let mut new_line = false;
    let mut is_string = false;

    while x1 < len {
        let c = chars.next().unwrap();
        if c == '"' {
            is_string = !is_string;
        }
        if !is_string && is_limiter(&c) {
            if x0 != x1 {
                let s = input.get(x0..x1).unwrap();
                if s == "//" {
                    skip_line = true;
                } else if !skip_line {
                    lexems.push(InputSlice::new(s.to_string(), column - s.len() as u32, row));
                }
                x0 = x1 + 1;
            } else {
                x0 += 1;
            }
            if !is_string
                && (c == ';'
                    || c == '('
                    || c == ')'
                    || c == '{'
                    || c == '}'
                    || c == ','
                    || c == '.'
                    || c == ':')
            {
                lexems.push(InputSlice::new(c.to_string(), column as u32, row));
            }
            if c == '\n' {
                skip_line = false;
                new_line = true;
                row += 1;
                column = 0;
            }
        }
        if new_line {
            new_line = false
        } else {
            column += 1;
        }
        x1 += 1;
    }
    if x0 != x1 {
        let s = input.get(x0..x1).unwrap();
        lexems.push(InputSlice::new(s.to_string(), column - s.len() as u32, row));
    }
    lexems
}

fn is_limiter(limiter: &char) -> bool {
    for l in LIMITERS.iter() {
        if limiter == l {
            return true;
        }
    }
    false
}
