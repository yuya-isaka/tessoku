fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let n = s.trim().parse::<i32>().unwrap();

    for i in (0..10).rev() {
        if ((n >> i) & 1) == 1 {
            print!("1");
        } else {
            print!("0");
        }
    }
}
