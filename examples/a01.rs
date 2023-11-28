fn main() {
    let mut input_string: String = String::new();
    std::io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    let trimmed_input: &str = input_string.trim();

    let parsed_number: i32 = match trimmed_input.parse() {
        Ok(n) => n,
        Err(_) => panic!("{trimmed_input} is not a number"),
    };

    let result: i32 = parsed_number * parsed_number;
    println!("{result}");
}
