fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let n: u16 = s.trim().parse().unwrap();
    let mut res = String::new();
    for i in (0..=9).rev() {
        res.push(if (n >> i) & 1 == 1 { '1' } else { '0' });
    }
    println!("{res}");
}

// わかりやすいコードではないが、短くしたらこう書ける
