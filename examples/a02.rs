fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let v: Vec<u32> = s
        .split_whitespace()
        .map(|tmp| tmp.parse().unwrap())
        .collect();
    let k = v.get(1).unwrap();
    s.clear();
    std::io::stdin().read_line(&mut s).unwrap();
    let a: Vec<u32> = s
        .split_whitespace()
        .map(|tmp| tmp.parse().unwrap())
        .collect();
    for tmp in &a {
        if tmp == k {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
