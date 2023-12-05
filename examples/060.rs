// これはアルゴリズム数学の問題

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let n: u64 = s.trim().parse().unwrap();

    if n % 4 == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}
