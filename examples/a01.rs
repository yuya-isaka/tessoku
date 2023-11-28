fn main() {
    let mut input_string = String::new();
    std::io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    let trimmed_input = input_string.trim();

    let parsed_number = match trimmed_input.parse::<i32>() {
        Ok(n) => n,
        Err(_) => panic!("{} is not a number", trimmed_input),
    };

    println!("{}", parsed_number * parsed_number);
}
