fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let n: u16 = s.trim().parse().unwrap();
    for i in (0..=9).rev() {
        print!("{}", (n >> i) & 1);
    }
}
