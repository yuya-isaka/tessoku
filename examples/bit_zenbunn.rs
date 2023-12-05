fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let nk: Vec<u32> = s
        .split_whitespace()
        .map(|tmp| tmp.parse().unwrap())
        .collect();
    let n = *nk.first().unwrap();
    let k = *nk.get(1).unwrap();
    s.clear();
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let a: Vec<u32> = s
        .split_whitespace()
        .map(|tmp| tmp.parse().unwrap())
        .collect();

    for i in 0..(1 << n) {
        let mut tmp = 0;
        for j in 0..n {
            if (i >> j & 1) == 1 {
                tmp += a[j as usize];
            }
        }
        if tmp == k {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
