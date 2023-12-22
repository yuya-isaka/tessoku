fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let nk = s
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let n = nk[0];
    let k = nk[1];

    let mut count = 0;
    for i in 1..=n {
        for j in 1..=n {
            let check = k - i - j;
            if 1 <= check && check <= n {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
