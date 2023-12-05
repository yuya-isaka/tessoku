fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let n: u32 = s.trim().parse().unwrap();
    for i in (0..10).rev() {
        print!("{}", n >> i & 1);
    }
}
