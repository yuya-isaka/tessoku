fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let nx = s.split_whitespace().collect::<Vec<_>>();
    let x = nx[1];

    let mut alist = String::new();
    std::io::stdin().read_line(&mut alist).unwrap();
    for a in alist.split_whitespace() {
        if a == x {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
