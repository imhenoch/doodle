#[derive(Clone)]
pub struct Expression {
    pub production: String,
    pub predict: Vec<String>,
    pub expression: Vec<String>,
}

impl Expression {
    fn new(production: String, predict: Vec<String>, expression: Vec<String>) -> Expression {
        Expression {
            production: production,
            predict: predict,
            expression: expression,
        }
    }
}

pub fn get_predict_set() -> Vec<Expression> {
    let mut predict_set = Vec::new();

    // # 1
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("fn"));
    expression.push(String::from("<code'>"));
    expression.push(String::from("<code>"));
    predict_set.push(Expression::new(String::from("<code>"), predict, expression));

    // # 2
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from(""));
    expression.push(String::from(""));
    predict_set.push(Expression::new(String::from("<code>"), predict, expression));

    // # 3
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("fn"));
    expression.push(String::from("<function_declaration>"));
    expression.push(String::from("<function_body>"));
    predict_set.push(Expression::new(
        String::from("<code'>"),
        predict,
        expression,
    ));

    // # 4
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("fn"));
    expression.push(String::from("<function_naming>"));
    expression.push(String::from("<function_return>"));
    predict_set.push(Expression::new(
        String::from("<function_declaration>"),
        predict,
        expression,
    ));

    // # 5
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("fn"));
    expression.push(String::from("<fn_identifier>"));
    expression.push(String::from("<function_param>"));
    predict_set.push(Expression::new(
        String::from("<function_naming>"),
        predict,
        expression,
    ));

    // # 6
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("fn"));
    expression.push(String::from("<fn>"));
    expression.push(String::from("<identifier>"));
    predict_set.push(Expression::new(
        String::from("<fn_identifier>"),
        predict,
        expression,
    ));

    // # 7
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("("));
    expression.push(String::from("<oparenthesis>"));
    expression.push(String::from("<function_params>"));
    predict_set.push(Expression::new(
        String::from("<function_param>"),
        predict,
        expression,
    ));

    // # 8
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from(""));
    expression.push(String::from(""));
    predict_set.push(Expression::new(
        String::from("<function_param>"),
        predict,
        expression,
    ));

    // # 9
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("identifier"));
    expression.push(String::from("<params>"));
    expression.push(String::from("<cparenthesis>"));
    predict_set.push(Expression::new(
        String::from("<function_params>"),
        predict,
        expression,
    ));

    // # 10
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from(")"));
    expression.push(String::from(")"));
    predict_set.push(Expression::new(
        String::from("<function_params>"),
        predict,
        expression,
    ));

    // # 11
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("identifier"));
    expression.push(String::from("<variable>"));
    expression.push(String::from("<params'>"));
    predict_set.push(Expression::new(
        String::from("<params>"),
        predict,
        expression,
    ));

    // # 12
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("identifier"));
    expression.push(String::from("<comma_variable>"));
    expression.push(String::from("<params>"));
    predict_set.push(Expression::new(
        String::from("<params'>"),
        predict,
        expression,
    ));

    // # 13
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from(","));
    expression.push(String::from("<comma>"));
    expression.push(String::from("<params>"));
    predict_set.push(Expression::new(
        String::from("<params'>"),
        predict,
        expression,
    ));

    // # 14
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from(""));
    expression.push(String::from(""));
    predict_set.push(Expression::new(
        String::from("<params'>"),
        predict,
        expression,
    ));

    // # 15
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("identifier"));
    expression.push(String::from("<variable>"));
    expression.push(String::from("<comma>"));
    predict_set.push(Expression::new(
        String::from("<comma_variable>"),
        predict,
        expression,
    ));

    // # 16
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from(":"));
    expression.push(String::from("<colon>"));
    expression.push(String::from("<return_type>"));
    predict_set.push(Expression::new(
        String::from("<function_return>"),
        predict,
        expression,
    ));

    // # 17
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from(""));
    expression.push(String::from(""));
    predict_set.push(Expression::new(
        String::from("<function_return>"),
        predict,
        expression,
    ));

    // # 18
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("int"));
    expression.push(String::from("int"));
    predict_set.push(Expression::new(
        String::from("<return_type>"),
        predict,
        expression,
    ));

    // # 19
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("float"));
    expression.push(String::from("float"));
    predict_set.push(Expression::new(
        String::from("<return_type>"),
        predict,
        expression,
    ));

    // # 20
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("char"));
    expression.push(String::from("char"));
    predict_set.push(Expression::new(
        String::from("<return_type>"),
        predict,
        expression,
    ));

    // # 21
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("str"));
    expression.push(String::from("str"));
    predict_set.push(Expression::new(
        String::from("<return_type>"),
        predict,
        expression,
    ));

    // # 22
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("bool"));
    expression.push(String::from("bool"));
    predict_set.push(Expression::new(
        String::from("<return_type>"),
        predict,
        expression,
    ));

    // # 23
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("{"));
    expression.push(String::from("<bracket_function>"));
    expression.push(String::from("<cbracket>"));
    predict_set.push(Expression::new(
        String::from("<function_body>"),
        predict,
        expression,
    ));

    // # 24
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("{"));
    expression.push(String::from("<obracket>"));
    expression.push(String::from("<function_content>"));
    predict_set.push(Expression::new(
        String::from("<bracket_function>"),
        predict,
        expression,
    ));

    // # 25
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("let"));
    predict.push(String::from("lem"));
    predict.push(String::from("call"));
    predict.push(String::from("for"));
    predict.push(String::from("ret"));
    predict.push(String::from("identifier"));
    predict.push(String::from("while"));
    predict.push(String::from("if"));
    expression.push(String::from("<function_contents>"));
    expression.push(String::from("<function_content>"));
    predict_set.push(Expression::new(
        String::from("<function_content>"),
        predict,
        expression,
    ));

    // # 26
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from(""));
    expression.push(String::from(""));
    predict_set.push(Expression::new(
        String::from("<function_content>"),
        predict,
        expression,
    ));

    // # 27
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("let"));
    predict.push(String::from("lem"));
    expression.push(String::from("<variable_declaration>"));
    expression.push(String::from("<semicolon>"));
    predict_set.push(Expression::new(
        String::from("<function_contents>"),
        predict,
        expression,
    ));

    // # 28
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("identifier"));
    expression.push(String::from("<assignment>"));
    expression.push(String::from("<semicolon>"));
    predict_set.push(Expression::new(
        String::from("<function_contents>"),
        predict,
        expression,
    ));

    // # 29
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("call"));
    expression.push(String::from("<function_call>"));
    expression.push(String::from("<semicolon>"));
    predict_set.push(Expression::new(
        String::from("<function_contents>"),
        predict,
        expression,
    ));

    // # 30
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("if"));
    expression.push(String::from("<if_declaration>"));
    expression.push(String::from("<else_declaration>"));
    predict_set.push(Expression::new(
        String::from("<function_contents>"),
        predict,
        expression,
    ));

    // # 31
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("for"));
    expression.push(String::from("<for_declaration>"));
    expression.push(String::from("<flow_body>"));
    predict_set.push(Expression::new(
        String::from("<function_contents>"),
        predict,
        expression,
    ));

    // # 32
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("while"));
    expression.push(String::from("<while_declaration>"));
    expression.push(String::from("<flow_body>"));
    predict_set.push(Expression::new(
        String::from("<function_contents>"),
        predict,
        expression,
    ));

    // # 33
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("ret"));
    expression.push(String::from("<ret>"));
    expression.push(String::from("<return>"));
    predict_set.push(Expression::new(
        String::from("<function_contents>"),
        predict,
        expression,
    ));

    // # 34
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from(""));
    expression.push(String::from(""));
    predict_set.push(Expression::new(
        String::from("<function_contents>"),
        predict,
        expression,
    ));

    // # 35
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("let"));
    expression.push(String::from("<let>"));
    expression.push(String::from("<multiple_variable>"));
    predict_set.push(Expression::new(
        String::from("<variable_declaration>"),
        predict,
        expression,
    ));

    // # 36
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("lem"));
    expression.push(String::from("<lem>"));
    expression.push(String::from("<multiple_variable>"));
    predict_set.push(Expression::new(
        String::from("<variable_declaration>"),
        predict,
        expression,
    ));

    // # 37
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("identifier"));
    expression.push(String::from("<variable>"));
    expression.push(String::from("<equals_assignment>"));
    predict_set.push(Expression::new(
        String::from("<single_variable>"),
        predict,
        expression,
    ));

    // # 38
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("="));
    expression.push(String::from("<equals>"));
    expression.push(String::from("<assignment'>"));
    predict_set.push(Expression::new(
        String::from("<equals_assignment>"),
        predict,
        expression,
    ));

    // # 39
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("identifier"));
    expression.push(String::from("<single_variable>"));
    expression.push(String::from("<multiple_variable'>"));
    predict_set.push(Expression::new(
        String::from("<multiple_variable>"),
        predict,
        expression,
    ));

    // # 40
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from(","));
    expression.push(String::from("<comma>"));
    expression.push(String::from("<multiple_variable>"));
    predict_set.push(Expression::new(
        String::from("<multiple_variable'>"),
        predict,
        expression,
    ));

    // # 41
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from(""));
    expression.push(String::from(""));
    predict_set.push(Expression::new(
        String::from("<multiple_variable'>"),
        predict,
        expression,
    ));

    // # 42
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("identifier"));
    expression.push(String::from("<identifier>"));
    expression.push(String::from("<variable_type>"));
    predict_set.push(Expression::new(
        String::from("<variable>"),
        predict,
        expression,
    ));

    // # 43
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from(":"));
    expression.push(String::from("<colon>"));
    expression.push(String::from("<data_type>"));
    predict_set.push(Expression::new(
        String::from("<variable_type>"),
        predict,
        expression,
    ));

    // # 44
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("identifier"));
    expression.push(String::from("<id_equals>"));
    expression.push(String::from("<assignment'>"));
    predict_set.push(Expression::new(
        String::from("<assignment>"),
        predict,
        expression,
    ));

    // # 45
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("identifier"));
    expression.push(String::from("<identifier>"));
    expression.push(String::from("<equals>"));
    predict_set.push(Expression::new(
        String::from("<id_equals>"),
        predict,
        expression,
    ));

    // # 46
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("value"));
    expression.push(String::from("<value>"));
    expression.push(String::from("<operator>"));
    predict_set.push(Expression::new(
        String::from("<assignment'>"),
        predict,
        expression,
    ));

    // # 47
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("identifier"));
    expression.push(String::from("<identifier>"));
    expression.push(String::from("<operator>"));
    predict_set.push(Expression::new(
        String::from("<assignment'>"),
        predict,
        expression,
    ));

    // # 48
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("call"));
    expression.push(String::from("<call_identifier>"));
    expression.push(String::from("<call_body>"));
    predict_set.push(Expression::new(
        String::from("<assignment'>"),
        predict,
        expression,
    ));

    // # 49
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("value"));
    expression.push(String::from("<value>"));
    expression.push(String::from("<operator>"));
    predict_set.push(Expression::new(
        String::from("<operation>"),
        predict,
        expression,
    ));

    // # 50
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("identifier"));
    expression.push(String::from("<identifier>"));
    expression.push(String::from("<operator>"));
    predict_set.push(Expression::new(
        String::from("<operation>"),
        predict,
        expression,
    ));

    // # 51
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("+"));
    predict.push(String::from("-"));
    predict.push(String::from("*"));
    predict.push(String::from("/"));
    predict.push(String::from("%"));
    predict.push(String::from("^"));
    predict.push(String::from("&&"));
    predict.push(String::from("||"));
    predict.push(String::from("=="));
    predict.push(String::from("!="));
    predict.push(String::from(">"));
    predict.push(String::from("<"));
    predict.push(String::from(">="));
    predict.push(String::from("<="));
    expression.push(String::from("<operator_char>"));
    expression.push(String::from("<operation>"));
    predict_set.push(Expression::new(
        String::from("<operator>"),
        predict,
        expression,
    ));

    // # 52
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from(""));
    expression.push(String::from(""));
    predict_set.push(Expression::new(
        String::from("<operator>"),
        predict,
        expression,
    ));

    // # 53
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("call"));
    expression.push(String::from("<call_identifier>"));
    expression.push(String::from("<call_body>"));
    predict_set.push(Expression::new(
        String::from("<function_call>"),
        predict,
        expression,
    ));

    // # 54
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("call"));
    expression.push(String::from("<call>"));
    expression.push(String::from("<identifier>"));
    predict_set.push(Expression::new(
        String::from("<call_identifier>"),
        predict,
        expression,
    ));

    // # 55
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("("));
    expression.push(String::from("<oparenthesis>"));
    expression.push(String::from("<parenthesis_call>"));
    predict_set.push(Expression::new(
        String::from("<call_body>"),
        predict,
        expression,
    ));

    // # 56
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from(""));
    expression.push(String::from(""));
    predict_set.push(Expression::new(
        String::from("<call_body>"),
        predict,
        expression,
    ));

    // # 57
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("value"));
    predict.push(String::from("identifier"));
    expression.push(String::from("<call_params>"));
    expression.push(String::from("<cparenthesis>"));
    predict_set.push(Expression::new(
        String::from("<parenthesis_call>"),
        predict,
        expression,
    ));

    // # 58
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("value"));
    predict.push(String::from("identifier"));
    expression.push(String::from("<operation>"));
    expression.push(String::from("<multiple_call_params>"));
    predict_set.push(Expression::new(
        String::from("<call_params>"),
        predict,
        expression,
    ));

    // # 59
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from(","));
    expression.push(String::from("<comma>"));
    expression.push(String::from("<call_params>"));
    predict_set.push(Expression::new(
        String::from("<multiple_call_params>"),
        predict,
        expression,
    ));

    // # 60
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from(""));
    expression.push(String::from(""));
    predict_set.push(Expression::new(
        String::from("<multiple_call_params>"),
        predict,
        expression,
    ));

    // # 61
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("if"));
    expression.push(String::from("<if_condition>"));
    expression.push(String::from("<flow_body>"));
    predict_set.push(Expression::new(
        String::from("<if_declaration>"),
        predict,
        expression,
    ));

    // # 62
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("if"));
    expression.push(String::from("<if>"));
    expression.push(String::from("<operation>"));
    predict_set.push(Expression::new(
        String::from("<if_condition>"),
        predict,
        expression,
    ));

    // # 63
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("else"));
    expression.push(String::from("<else>"));
    expression.push(String::from("<flow_body>"));
    predict_set.push(Expression::new(
        String::from("<else_declaration>"),
        predict,
        expression,
    ));

    // # 64
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("for"));
    expression.push(String::from("<for>"));
    expression.push(String::from("<for_condition>"));
    predict_set.push(Expression::new(
        String::from("<for_declaration>"),
        predict,
        expression,
    ));

    // # 65
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("identifier"));
    expression.push(String::from("<id_in>"));
    expression.push(String::from("<range>"));
    predict_set.push(Expression::new(
        String::from("<for_condition>"),
        predict,
        expression,
    ));

    // # 66
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("identifier"));
    expression.push(String::from("<identifier>"));
    expression.push(String::from("<in>"));
    predict_set.push(Expression::new(
        String::from("<id_in>"),
        predict,
        expression,
    ));

    // # 67
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("value"));
    expression.push(String::from("<value_to>"));
    expression.push(String::from("<range_value>"));
    predict_set.push(Expression::new(
        String::from("<range>"),
        predict,
        expression,
    ));

    // # 68
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("identifier"));
    expression.push(String::from("<identifier_to>"));
    expression.push(String::from("<range_identifier>"));
    predict_set.push(Expression::new(
        String::from("<range>"),
        predict,
        expression,
    ));

    // # 69
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("value"));
    expression.push(String::from("value"));
    predict_set.push(Expression::new(
        String::from("<range_value>"),
        predict,
        expression,
    ));

    // # 70
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("identifier"));
    expression.push(String::from("identifier"));
    predict_set.push(Expression::new(
        String::from("<range_value>"),
        predict,
        expression,
    ));

    // # 71
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("value"));
    expression.push(String::from("value"));
    predict_set.push(Expression::new(
        String::from("<range_identifier>"),
        predict,
        expression,
    ));

    // # 72
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("identifier"));
    expression.push(String::from("identifier"));
    predict_set.push(Expression::new(
        String::from("<range_identifier>"),
        predict,
        expression,
    ));

    // # 73
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("value"));
    expression.push(String::from("<value>"));
    expression.push(String::from("<to>"));
    predict_set.push(Expression::new(
        String::from("<value_to>"),
        predict,
        expression,
    ));

    // # 74
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("identifier"));
    expression.push(String::from("<identifier>"));
    expression.push(String::from("<to>"));
    predict_set.push(Expression::new(
        String::from("<identifier_to>"),
        predict,
        expression,
    ));

    // # 75
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("while"));
    expression.push(String::from("<while>"));
    expression.push(String::from("<operation>"));
    predict_set.push(Expression::new(
        String::from("<while_declaration>"),
        predict,
        expression,
    ));

    // # 76
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("{"));
    expression.push(String::from("<obracket>"));
    expression.push(String::from("<bracket_flow>"));
    predict_set.push(Expression::new(
        String::from("<flow_body>"),
        predict,
        expression,
    ));

    // # 77
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from(""));
    predict.push(String::from("identifier"));
    predict.push(String::from("if"));
    predict.push(String::from("while"));
    predict.push(String::from("call"));
    predict.push(String::from("for"));
    expression.push(String::from("<flow_content>"));
    expression.push(String::from("<cbracket>"));
    predict_set.push(Expression::new(
        String::from("<bracket_flow>"),
        predict,
        expression,
    ));

    // # 78
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("identifier"));
    predict.push(String::from("if"));
    predict.push(String::from("while"));
    predict.push(String::from("call"));
    predict.push(String::from("for"));
    expression.push(String::from("<flow_contents>"));
    expression.push(String::from("<flow_content>"));
    predict_set.push(Expression::new(
        String::from("<flow_content>"),
        predict,
        expression,
    ));

    // # 79
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from(""));
    expression.push(String::from(""));
    predict_set.push(Expression::new(
        String::from("<flow_content>"),
        predict,
        expression,
    ));

    // # 80
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("identifier"));
    expression.push(String::from("<assignment>"));
    expression.push(String::from("<semicolon>"));
    predict_set.push(Expression::new(
        String::from("<flow_contents>"),
        predict,
        expression,
    ));

    // # 81
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("call"));
    expression.push(String::from("<function_call>"));
    expression.push(String::from("<semicolon>"));
    predict_set.push(Expression::new(
        String::from("<flow_contents>"),
        predict,
        expression,
    ));

    // # 82
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("if"));
    expression.push(String::from("<if_declaration>"));
    expression.push(String::from("<else_declaration>"));
    predict_set.push(Expression::new(
        String::from("<flow_contents>"),
        predict,
        expression,
    ));

    // # 83
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("for"));
    expression.push(String::from("<for_declaration>"));
    expression.push(String::from("<flow_body>"));
    predict_set.push(Expression::new(
        String::from("<flow_contents>"),
        predict,
        expression,
    ));

    // # 84
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("while"));
    expression.push(String::from("<while_declaration>"));
    expression.push(String::from("<flow_body>"));
    predict_set.push(Expression::new(
        String::from("<flow_contents>"),
        predict,
        expression,
    ));

    // # 85
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from(""));
    expression.push(String::from(""));
    predict_set.push(Expression::new(
        String::from("<flow_contents>"),
        predict,
        expression,
    ));

    // # 86
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("value"));
    predict.push(String::from("identifier"));
    expression.push(String::from("<operation>"));
    expression.push(String::from("<semicolon>"));
    predict_set.push(Expression::new(
        String::from("<return>"),
        predict,
        expression,
    ));

    // # 87
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("int"));
    expression.push(String::from("int"));
    predict_set.push(Expression::new(
        String::from("<data_type>"),
        predict,
        expression,
    ));

    // # 88
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("float"));
    expression.push(String::from("float"));
    predict_set.push(Expression::new(
        String::from("<data_type>"),
        predict,
        expression,
    ));

    // # 89
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("char"));
    expression.push(String::from("char"));
    predict_set.push(Expression::new(
        String::from("<data_type>"),
        predict,
        expression,
    ));

    // # 90
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("str"));
    expression.push(String::from("str"));
    predict_set.push(Expression::new(
        String::from("<data_type>"),
        predict,
        expression,
    ));

    // # 91
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("bool"));
    expression.push(String::from("bool"));
    predict_set.push(Expression::new(
        String::from("<data_type>"),
        predict,
        expression,
    ));

    // # 92
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("+"));
    expression.push(String::from("+"));
    predict_set.push(Expression::new(
        String::from("<operator_char>"),
        predict,
        expression,
    ));

    // # 93
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("-"));
    expression.push(String::from("-"));
    predict_set.push(Expression::new(
        String::from("<operator_char>"),
        predict,
        expression,
    ));

    // # 94
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("*"));
    expression.push(String::from("*"));
    predict_set.push(Expression::new(
        String::from("<operator_char>"),
        predict,
        expression,
    ));

    // # 95
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("/"));
    expression.push(String::from("/"));
    predict_set.push(Expression::new(
        String::from("<operator_char>"),
        predict,
        expression,
    ));

    // # 96
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("%"));
    expression.push(String::from("%"));
    predict_set.push(Expression::new(
        String::from("<operator_char>"),
        predict,
        expression,
    ));

    // # 97
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("^"));
    expression.push(String::from("^"));
    predict_set.push(Expression::new(
        String::from("<operator_char>"),
        predict,
        expression,
    ));

    // # 98
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("&&"));
    expression.push(String::from("&&"));
    predict_set.push(Expression::new(
        String::from("<operator_char>"),
        predict,
        expression,
    ));

    // # 99
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("||"));
    expression.push(String::from("||"));
    predict_set.push(Expression::new(
        String::from("<operator_char>"),
        predict,
        expression,
    ));

    // # 100
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("=="));
    expression.push(String::from("=="));
    predict_set.push(Expression::new(
        String::from("<operator_char>"),
        predict,
        expression,
    ));

    // # 101
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("!="));
    expression.push(String::from("!="));
    predict_set.push(Expression::new(
        String::from("<operator_char>"),
        predict,
        expression,
    ));

    // # 102
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from(">"));
    expression.push(String::from(">"));
    predict_set.push(Expression::new(
        String::from("<operator_char>"),
        predict,
        expression,
    ));

    // # 103
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("<"));
    expression.push(String::from("<"));
    predict_set.push(Expression::new(
        String::from("<operator_char>"),
        predict,
        expression,
    ));

    // # 104
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from(">="));
    expression.push(String::from(">="));
    predict_set.push(Expression::new(
        String::from("<operator_char>"),
        predict,
        expression,
    ));

    // # 105
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("<="));
    expression.push(String::from("<="));
    predict_set.push(Expression::new(
        String::from("<operator_char>"),
        predict,
        expression,
    ));

    // # 106
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("fn"));
    expression.push(String::from("fn"));
    predict_set.push(Expression::new(String::from("<fn>"), predict, expression));

    // # 107
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("call"));
    expression.push(String::from("call"));
    predict_set.push(Expression::new(String::from("<call>"), predict, expression));

    // # 108
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("let"));
    expression.push(String::from("let"));
    predict_set.push(Expression::new(String::from("<let>"), predict, expression));

    // # 109
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("lem"));
    expression.push(String::from("lem"));
    predict_set.push(Expression::new(String::from("<lem>"), predict, expression));

    // # 110
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("if"));
    expression.push(String::from("if"));
    predict_set.push(Expression::new(String::from("<if>"), predict, expression));

    // # 111
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("else"));
    expression.push(String::from("else"));
    predict_set.push(Expression::new(String::from("<else>"), predict, expression));

    // # 112
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("for"));
    expression.push(String::from("for"));
    predict_set.push(Expression::new(String::from("<for>"), predict, expression));

    // # 113
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("while"));
    expression.push(String::from("while"));
    predict_set.push(Expression::new(
        String::from("<while>"),
        predict,
        expression,
    ));

    // # 114
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("in"));
    expression.push(String::from("in"));
    predict_set.push(Expression::new(String::from("<in>"), predict, expression));

    // # 115
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("to"));
    expression.push(String::from("to"));
    predict_set.push(Expression::new(String::from("<to>"), predict, expression));

    // # 116
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("ret"));
    expression.push(String::from("ret"));
    predict_set.push(Expression::new(String::from("<ret>"), predict, expression));

    // # 117
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("identifier"));
    expression.push(String::from("identifier"));
    predict_set.push(Expression::new(
        String::from("<identifier>"),
        predict,
        expression,
    ));

    // # 118
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("value"));
    expression.push(String::from("value"));
    predict_set.push(Expression::new(
        String::from("<value>"),
        predict,
        expression,
    ));

    // # 119
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("("));
    expression.push(String::from("("));
    predict_set.push(Expression::new(
        String::from("<oparenthesis>"),
        predict,
        expression,
    ));

    // # 120
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from(")"));
    expression.push(String::from(")"));
    predict_set.push(Expression::new(
        String::from("<cparenthesis>"),
        predict,
        expression,
    ));

    // # 121
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("{"));
    expression.push(String::from("{"));
    predict_set.push(Expression::new(
        String::from("<obracket>"),
        predict,
        expression,
    ));

    // # 122
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("}"));
    expression.push(String::from("}"));
    predict_set.push(Expression::new(
        String::from("<cbracket>"),
        predict,
        expression,
    ));

    // # 123
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from(":"));
    expression.push(String::from(":"));
    predict_set.push(Expression::new(
        String::from("<colon>"),
        predict,
        expression,
    ));

    // # 124
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from(";"));
    expression.push(String::from(";"));
    predict_set.push(Expression::new(
        String::from("<semicolon>"),
        predict,
        expression,
    ));

    // # 125
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from(","));
    expression.push(String::from(","));
    predict_set.push(Expression::new(
        String::from("<comma>"),
        predict,
        expression,
    ));

    // # 126
    let mut predict = Vec::new();
    let mut expression = Vec::new();
    predict.push(String::from("="));
    expression.push(String::from("="));
    predict_set.push(Expression::new(
        String::from("<equals>"),
        predict,
        expression,
    ));

    predict_set
}
