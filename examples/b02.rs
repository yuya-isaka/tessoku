fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let ab: Vec<u32> = s
        .split_whitespace()
        .map(|tmp| tmp.parse().unwrap())
        .collect();
    let a: u32 = *ab.first().unwrap();
    let b: u32 = *ab.get(1).unwrap();

    for tmp in a..=b {
        if 100 % tmp == 0 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
