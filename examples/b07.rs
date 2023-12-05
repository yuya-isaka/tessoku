fn main() {
    let t = input() as usize;
    let n = input();

    let mut jikoku = vec![0; t + 2];
    for _ in 0..n {
        let lr = input_array();
        let l = *lr.first().unwrap();
        let r = *lr.get(1).unwrap();
        jikoku[l + 1] += 1;
        jikoku[r + 1] -= 1;
    }

    for i in 1..=t {
        jikoku[i] += jikoku[i - 1];
        println!("{}", jikoku[i]);
    }
}

fn input() -> u64 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    return s.trim().parse().unwrap();
}

fn input_array() -> Vec<usize> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    return s
        .split_whitespace()
        .map(|tmp| tmp.parse().unwrap())
        .collect();
}
