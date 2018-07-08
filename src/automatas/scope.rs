use automatas::{Category, Lexem};

pub fn set_scope(lexems: &mut Vec<Lexem>) {
    let mut scope = 0;
    let mut id = 0;
    let mut braces: Vec<String> = Vec::new();
    for lexem in lexems {
        if lexem.token == "fn" && braces.is_empty() {
            id = 1;
            scope += 1;
        } else if lexem.token == "{" {
            braces.push(lexem.token.clone());
        } else if lexem.token == "}" {
            braces.pop();
        }

        match lexem.category {
            Category::ID => lexem.set_scope(if id == 0 {
                scope.to_string()
            } else {
                id = 0;
                id.to_string()
            }),
            _ => {}
        }
    }
}
