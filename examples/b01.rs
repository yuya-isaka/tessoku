fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let ab = s
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    println!("{}", ab[0] + ab[1]);
}
