fn main() {
    let input_string: Vec<String> = input_array();
    let _n: u32 = input_string.get(0).unwrap().parse().unwrap();
    let x: u32 = input_string.get(1).unwrap().parse().unwrap();

    let input_string: Vec<String> = input_array();
    let a_list: Vec<u32> = input_string.iter().map(|s| s.parse().unwrap()).collect();

    if a_list.contains(&x) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn input_array() -> Vec<String> {
    let mut inputed_number: String = String::new();
    std::io::stdin().read_line(&mut inputed_number).unwrap();
    return inputed_number
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
}
