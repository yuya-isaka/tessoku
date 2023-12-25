fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let nq = s
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let n = nq[0];
    let q = nq[1];

    let mut s2 = String::new();
    std::io::stdin().read_line(&mut s2).unwrap();
    let alist = s2
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut ruisekiwa = vec![0; n + 1];
    for i in 0..n {
        ruisekiwa[i + 1] = ruisekiwa[i] + alist[i];
    }

    for _ in 0..q {
        let mut s3 = String::new();
        std::io::stdin().read_line(&mut s3).unwrap();
        let lr = s3
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let l = lr[0];
        let r = lr[1];

        println!("{}", ruisekiwa[r] - ruisekiwa[l - 1]);
    }
}
