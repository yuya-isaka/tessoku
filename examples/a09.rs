fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let hwn = s
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let h = hwn[0];
    let w = hwn[1];
    let n = hwn[2];

    let mut ruisekiwa = vec![vec![0; w + 1]; h];

    for _ in 0..n {
        s.clear();
        std::io::stdin().read_line(&mut s).unwrap();
        let abcd = s
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let a = abcd[0] - 1;
        let b = abcd[1] - 1;
        let c = abcd[2] - 1;
        let d = abcd[3] - 1;

        for line in ruisekiwa.iter_mut().take(c + 1).skip(a) {
            line[b] += 1;
            line[d + 1] -= 1;
        }
    }

    for line in ruisekiwa.iter_mut().take(h) {
        for j in 1..w {
            line[j] += line[j - 1];
        }
    }

    for line in ruisekiwa.iter().take(h) {
        for data in line.iter().take(w) {
            print!("{} ", data);
        }
        println!();
    }
}
