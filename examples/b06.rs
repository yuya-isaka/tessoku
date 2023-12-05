fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let n: usize = s.trim().parse().unwrap();
    let a = input_array();
    s.clear();
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let q: u64 = s.trim().parse().unwrap();

    let mut ruisekiwa: Vec<i64> = vec![0; n + 1];
    for (i, v) in a.iter().enumerate() {
        match *v {
            1 => ruisekiwa[i + 1] = ruisekiwa[i] + 1,
            _ => ruisekiwa[i + 1] = ruisekiwa[i] - 1,
        }
    }

    for _ in 0..q {
        let lr = input_array();
        let l = *lr.first().unwrap() as usize;
        let r = *lr.get(1).unwrap() as usize;
        let tmp = ruisekiwa[r] - ruisekiwa[l - 1];
        match tmp {
            0 => println!("draw"),
            n if n > 0 => println!("win"),
            _ => println!("lose"),
        }
    }
}

fn input_array() -> Vec<u64> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    return s
        .split_whitespace()
        .map(|tmp| tmp.parse().unwrap())
        .collect();
}
