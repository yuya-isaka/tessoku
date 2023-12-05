fn main() {
    let nq = input_array();
    let n = *nq.first().unwrap() as usize;
    let q = *nq.get(1).unwrap();
    let a = input_array();

    let mut ruisekiwa: Vec<u64> = vec![0; n + 1];
    for (i, v) in a.iter().enumerate() {
        ruisekiwa[i + 1] = ruisekiwa[i] + v;
    }

    for _ in 0..q {
        let lr = input_array();
        let l = *lr.first().unwrap() - 1;
        let r = *lr.get(1).unwrap();
        println!("{}", ruisekiwa[r as usize] - ruisekiwa[l as usize]);
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
