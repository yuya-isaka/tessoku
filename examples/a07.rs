fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let d: usize = s.trim().parse().unwrap();
    s.clear();
    std::io::stdin().read_line(&mut s).unwrap();
    let n: usize = s.trim().parse().unwrap();

    let mut ninnzuu: Vec<i64> = vec![0; d + 1];
    for _ in 0..n {
        let lr = input_array();
        let l = *lr.first().unwrap();
        let r = *lr.get(1).unwrap();
        ninnzuu[l - 1] += 1;
        ninnzuu[r] -= 1;
    }

    let mut ruisekiwa: Vec<i64> = vec![0; d + 1];
    for i in 1..=d {
        ruisekiwa[i] = ruisekiwa[i - 1] + ninnzuu[i - 1];
        println!("{}", ruisekiwa[i]);
    }
}

fn input_array() -> Vec<usize> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    return s
        .split_whitespace()
        .map(|tmp| tmp.parse().unwrap())
        .collect();
}
