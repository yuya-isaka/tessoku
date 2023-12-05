fn main() {
    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string).unwrap();
    let trimmed_input = input_string.trim();
    let parsed_input: u32 = trimmed_input.parse().unwrap();
    let result = parsed_input * parsed_input;
    println!("{result}");
}
