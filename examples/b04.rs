fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut result = 0;
    s.trim().chars().for_each(|tmp| {
        if tmp == '1' {
            result |= 1;
        }
        result <<= 1;
    });
    println!("{}", result >> 1);
}
