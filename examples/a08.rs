fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let hw = s
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let h = hw[0];
    let w = hw[1];

    let mut xlist = vec![];
    for _ in 0..h {
        s.clear();
        std::io::stdin().read_line(&mut s).unwrap();
        let alist = s
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        xlist.push(alist);
    }

    let mut ruisekiwa = vec![vec![0; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            ruisekiwa[i + 1][j + 1] += ruisekiwa[i + 1][j] + xlist[i][j];
        }
    }
    for j in 0..w {
        for i in 0..h {
            ruisekiwa[i + 1][j + 1] += ruisekiwa[i][j + 1];
        }
    }

    s.clear();
    std::io::stdin().read_line(&mut s).unwrap();
    let q = s.trim().parse::<usize>().unwrap();

    let mut abcd_list = vec![];
    for _ in 0..q {
        s.clear();
        std::io::stdin().read_line(&mut s).unwrap();
        let abcd = s
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        abcd_list.push(abcd);
    }

    for data in abcd_list {
        let a = data[0];
        let b = data[1];
        let c = data[2];
        let d = data[3];

        let mut ans = ruisekiwa[c][d];
        if a > 1 {
            ans -= ruisekiwa[a - 1][d];
        }
        if b > 1 {
            ans -= ruisekiwa[c][b - 1];
        }
        if a > 1 && b > 1 {
            ans += ruisekiwa[a - 1][b - 1];
        }

        println!("{}", ans);
    }
}
