fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let d = s.trim().parse::<usize>().unwrap();

    s.clear();
    std::io::stdin().read_line(&mut s).unwrap();
    let n = s.trim().parse::<usize>().unwrap();

    let mut ruisekiwa = vec![0; d + 1];

    for _ in 0..n {
        s.clear();
        std::io::stdin().read_line(&mut s).unwrap();
        let lr = s
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let l = lr[0] - 1;
        let r = lr[1] - 1;

        ruisekiwa[l] += 1;
        ruisekiwa[r + 1] -= 1;
    }

    let mut ruisekiwa2 = vec![0; d + 1];
    for i in 1..=d {
        ruisekiwa2[i] = ruisekiwa2[i - 1] + ruisekiwa[i - 1];
    }

    for data in ruisekiwa2.iter().take(d + 1).skip(1) {
        println!("{}", data);
    }
}
