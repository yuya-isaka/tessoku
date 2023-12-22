fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let ns = s
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let n = ns[0];
    let s = ns[1];

    let mut s2 = String::new();
    std::io::stdin().read_line(&mut s2).unwrap();
    let alist = s2
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    for i in 0..(1 << n) {
        let mut sum = 0;
        for j in 0..n {
            if i & (1 << j) != 0 {
                sum += alist[j as usize];
            }
        }
        if sum == s {
            println!("Yes");
            return;
        }
    }
    println!("No")
}
