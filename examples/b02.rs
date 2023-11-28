fn main() {
    let input_string: Vec<String> = input_array();
    let a: u32 = input_string.get(0).unwrap().parse().unwrap();
    let b: u32 = input_string.get(1).unwrap().parse().unwrap();

    for i in a..=b {
        if 100 % i == 0 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

fn input_array() -> Vec<String> {
    let mut inputed_number: String = String::new();
    std::io::stdin().read_line(&mut inputed_number).unwrap();
    return inputed_number
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
}