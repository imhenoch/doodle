use lexer::InputSlice;

pub fn get_symbol_table(input: Vec<InputSlice>) {
    let mut symbol_table: Vec<u32> = Vec::new();
    for slice in input {
        println!("{}", slice);
    }
}
