fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let nk: Vec<u32> = s
        .split_whitespace()
        .map(|tmp| tmp.parse().unwrap())
        .collect();
    let n = *nk.first().unwrap();
    let k = *nk.get(1).unwrap();

    let mut result = 0;
    for i in 1..=n {
        for j in 1..=n {
            if 1 <= k - i - j && k - i - j <= n {
                result += 1;
            }
        }
    }

    println!("{result}");
}
