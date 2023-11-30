fn main() {
    let nk = input_array();
    let n = *nk.first().unwrap();
    let k = *nk.get(1).unwrap();

    let a = input_array();

    for i in 1..(1 << n) {
        let mut res: u64 = 0;
        for (j, c) in (0..n).rev().enumerate() {
            let tmp = i >> c & 1;
            if tmp == 1 {
                res += a[j];
            }
        }
        if res == k {
            println!("Yes");
        }
    }

    println!("No");
}

fn input_array() -> Vec<u64> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
