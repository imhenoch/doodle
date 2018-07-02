use automatas::{Category, Lexem};

pub fn set_scope(lexems: &mut Vec<Lexem>) {
    let mut scope = 0;
    let mut braces: Vec<String> = Vec::new();
    for lexem in lexems {
        if lexem.token == "fn" && braces.is_empty() {
            scope += 1;
        } else if lexem.token == "{" {
            braces.push(lexem.token.clone());
        } else if lexem.token == "}" {
            braces.pop();
        }

        match lexem.category {
            Category::ID => lexem.set_scope(scope.to_string()),
            _ => {}
        }
    }
}
