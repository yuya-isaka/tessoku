fn main() {
    let inputed_string = input_array();
    let a: u32 = inputed_string.get(0).unwrap().parse().unwrap();
    let b: u32 = inputed_string.get(1).unwrap().parse().unwrap();

    println!("{}", a + b);
}

fn input_array() -> Vec<String> {
    let mut inputed_number: String = String::new();
    std::io::stdin().read_line(&mut inputed_number).unwrap();
    return inputed_number
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
}
