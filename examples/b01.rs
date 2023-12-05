fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let v: Vec<String> = s.split_whitespace().map(|tmp| tmp.to_string()).collect();
    let a: u32 = v.get(0).unwrap().parse().unwrap();
    let b: u32 = v.get(1).unwrap().parse().unwrap();
    let result = a + b;
    println!("{result}");
}
