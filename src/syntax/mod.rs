use automatas::{Category, Lexem};
use std::collections::LinkedList;

mod syntax_table;

pub fn syntax_analysis(symbols: &Vec<Lexem>) -> (Vec<String>, Vec<String>, String) {
    let mut error = false;
    let mut the_error = String::new();
    let mut input = LinkedList::new();
    let mut the_input = Vec::new();
    let mut the_stack = Vec::new();
    for symbol in symbols {
        input.push_back(match symbol.category {
            Category::ID => String::from("identifier"),
            Category::VALUE => String::from("value"),
            _ => symbol.token.clone(),
        })
    }
    input.push_back(String::from("$"));

    let predict_set = syntax_table::get_predict_set();

    let mut stack = LinkedList::new();
    stack.push_back(String::from("$"));
    stack.push_back(String::from("<code>"));

    loop {
        let input_clone = input.clone();
        let stack_clone = stack.clone();
        let mut input_string = String::new();
        let mut stack_string = String::new();
        for s in stack_clone {
            input_string.push_str(&format!("{} ", &s.to_string()));
            print!("{} ", s);
        }
        print!("   ->   ");
        for i in input_clone {
            stack_string.push_str(&format!("{} ", &i.to_string()));
            print!("{} ", i);
        }
        print!("\n");
        the_input.push(input_string);
        the_stack.push(stack_string);

        let predict_set = predict_set.clone();

        if input.front() == stack.back() {
            stack.pop_back();
            input.pop_front();
            if stack.is_empty() && input.is_empty() {
                break;
            }
        } else {
            let stack_top = stack.back().unwrap().to_string();
            let input_top = input.front().unwrap().to_string();
            let predict_set_clone = predict_set.clone();
            let mut filter = predict_set
                .into_iter()
                .filter(|set| set.production == stack_top && set.predict.contains(&input_top));
            let next = filter.next();
            match next {
                None => {
                    let mut filter = predict_set_clone.into_iter().filter(|set| {
                        set.production == stack_top && set.predict.contains(&String::from(""))
                    });
                    let another_next = filter.next();
                    match another_next {
                        None => {
                            error = true;
                            break;
                        }
                        _ => {
                            let mut expression = another_next.unwrap().expression;
                            expression.reverse();
                            stack.pop_back();
                            for ex in expression {
                                if !ex.is_empty() {
                                    stack.push_back(ex);
                                }
                            }
                        }
                    }
                }
                _ => {
                    let mut expression = next.unwrap().expression;
                    expression.reverse();
                    stack.pop_back();
                    for ex in expression {
                        if !ex.is_empty() {
                            stack.push_back(ex);
                        }
                    }
                }
            }
        }
    }

    if error {
        the_error.push_str(&format!("Error near {}", input.front().unwrap()));
    }
    println!(
        "{}\n",
        if error {
            the_error.clone()
        } else {
            "No errors".to_string()
        }
    );
    (the_stack, the_input, the_error)
}
