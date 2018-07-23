use std::collections::HashMap;

pub fn get_columns() -> HashMap<String, u32> {
    let mut columns = HashMap::new();
    columns.insert(String::from("fn"), 0);
    columns.insert(String::from("call"), 1);
    columns.insert(String::from("let"), 2);
    columns.insert(String::from("lem"), 3);
    columns.insert(String::from("if"), 4);
    columns.insert(String::from("else"), 5);
    columns.insert(String::from("for"), 6);
    columns.insert(String::from("while"), 7);
    columns.insert(String::from("in"), 8);
    columns.insert(String::from("to"), 9);
    columns.insert(String::from("ret"), 10);
    columns.insert(String::from("identifier"), 11);
    columns.insert(String::from("value"), 12);
    columns.insert(String::from("data_type"), 13);
    columns.insert(String::from("operator_char"), 14);
    columns.insert(String::from("("), 15);
    columns.insert(String::from(")"), 16);
    columns.insert(String::from("{"), 17);
    columns.insert(String::from("}"), 18);
    columns.insert(String::from(":"), 19);
    columns.insert(String::from(";"), 20);
    columns.insert(String::from(","), 21);
    columns.insert(String::from("="), 22);
    columns.insert(String::from("ε"), 23);
    columns
}

pub fn get_rows() -> HashMap<String, u32> {
    let mut rows = HashMap::new();
    rows.insert(String::from("<code>"), 0);
    rows.insert(String::from("<code'>"), 1);
    rows.insert(String::from("<function_declaration>"), 2);
    rows.insert(String::from("<function_naming>"), 3);
    rows.insert(String::from("<fn_identifier>"), 4);
    rows.insert(String::from("<function_param>"), 5);
    rows.insert(String::from("<function_params>"), 6);
    rows.insert(String::from("<params>"), 7);
    rows.insert(String::from("<params'>"), 8);
    rows.insert(String::from("<comma_variable>"), 9);
    rows.insert(String::from("<function_return>"), 10);
    rows.insert(String::from("<return_type>"), 11);
    rows.insert(String::from("<function_body>"), 12);
    rows.insert(String::from("<bracket_function>"), 13);
    rows.insert(String::from("<function_content>"), 14);
    rows.insert(String::from("<variable_declaration>"), 15);
    rows.insert(String::from("<single_variable>"), 16);
    rows.insert(String::from("<multiple_variable>"), 17);
    rows.insert(String::from("<variable>"), 18);
    rows.insert(String::from("<variable_type>"), 19);
    rows.insert(String::from("<assignment>"), 20);
    rows.insert(String::from("<id_equals>"), 21);
    rows.insert(String::from("<operation>"), 22);
    rows.insert(String::from("<operator>"), 23);
    rows.insert(String::from("<function_call>"), 24);
    rows.insert(String::from("<call_identifier>"), 25);
    rows.insert(String::from("<call_body>"), 26);
    rows.insert(String::from("<parenthesis_call>"), 27);
    rows.insert(String::from("<call_params>"), 28);
    rows.insert(String::from("<multiple_call_params>"), 29);
    rows.insert(String::from("<if_declaration>"), 30);
    rows.insert(String::from("<if_condition>"), 31);
    rows.insert(String::from("<else_declaration>"), 32);
    rows.insert(String::from("<for_declaration>"), 33);
    rows.insert(String::from("<for_condition>"), 34);
    rows.insert(String::from("<id_in>"), 35);
    rows.insert(String::from("<range>"), 36);
    rows.insert(String::from("<range_value>"), 37);
    rows.insert(String::from("<range_identifier>"), 38);
    rows.insert(String::from("<value_to>"), 39);
    rows.insert(String::from("<identifier_to>"), 40);
    rows.insert(String::from("<while_declaration>"), 41);
    rows.insert(String::from("<flow_body>"), 42);
    rows.insert(String::from("<bracket_flow>"), 43);
    rows.insert(String::from("<flow_content>"), 44);
    rows.insert(String::from("<flow_contents>"), 45);
    rows.insert(String::from("<return>"), 46);
    rows.insert(String::from("<data_type>"), 47);
    rows.insert(String::from("<operator_char>"), 48);
    rows.insert(String::from("<fn>"), 49);
    rows.insert(String::from("<call>"), 50);
    rows.insert(String::from("<let>"), 51);
    rows.insert(String::from("<lem>"), 52);
    rows.insert(String::from("<if>"), 53);
    rows.insert(String::from("<else>"), 54);
    rows.insert(String::from("<for>"), 55);
    rows.insert(String::from("<while>"), 56);
    rows.insert(String::from("<in>"), 57);
    rows.insert(String::from("<to>"), 58);
    rows.insert(String::from("<ret>"), 59);
    rows.insert(String::from("<identifier>"), 60);
    rows.insert(String::from("<value>"), 61);
    rows.insert(String::from("<oparenthesis>"), 62);
    rows.insert(String::from("<cparenthesis>"), 63);
    rows.insert(String::from("<obracket>"), 64);
    rows.insert(String::from("<cbracket>"), 65);
    rows.insert(String::from("<colon>"), 66);
    rows.insert(String::from("<semicolon>"), 67);
    rows.insert(String::from("<comma>"), 68);
    rows.insert(String::from("<equals>"), 69);
    rows
}

pub fn get_syntax_table() {}
