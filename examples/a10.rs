fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let n = s.trim().parse::<usize>().unwrap();
    s.clear();
    std::io::stdin().read_line(&mut s).unwrap();
    let alist = s
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut ruisekiwa1 = vec![0; n + 1];
    for i in 0..n {
        ruisekiwa1[i + 1] = std::cmp::max(ruisekiwa1[i], alist[i]);
    }
    // println!("ruisekiwa1");
    // for i in 0..n {
    //     print!("{} ", ruisekiwa1[i]);
    // }
    // println!("");

    let mut ruisekiwa2 = vec![0; n + 1];
    let alist_rev = alist.into_iter().rev().collect::<Vec<i32>>();
    for i in 0..n {
        ruisekiwa2[i + 1] = std::cmp::max(ruisekiwa2[i], alist_rev[i]);
    }
    // println!("ruisekiwa2");
    // for i in 0..n {
    //     print!("{} ", ruisekiwa2[i]);
    // }
    // println!("");

    s.clear();
    std::io::stdin().read_line(&mut s).unwrap();
    let d = s.trim().parse::<i32>().unwrap();

    for _ in 0..d {
        s.clear();
        std::io::stdin().read_line(&mut s).unwrap();
        let lr = s
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let l = (lr[0] - 1) as usize;
        let r = ((n as i32) - lr[1]) as usize;

        println!("{}", std::cmp::max(ruisekiwa1[l], ruisekiwa2[r]));
    }
}
