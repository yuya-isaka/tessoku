fn main() {
    let nk = input_line();
    let n: u32 = *nk.first().unwrap();
    let k: u32 = *nk.get(1).unwrap();

    let mut cnt = 0;
    for i in 1..=n {
        for j in 1..=n {
            if (i + j) >= k {
                continue;
            }
            if k - (i + j) <= n {
                cnt += 1;
            }
        }
    }

    println!("{cnt}");
}

fn input_line() -> Vec<u32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.split_whitespace()
        .map(|s| s.to_string().parse().unwrap())
        .collect()
}
