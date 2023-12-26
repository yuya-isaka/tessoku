fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let tmp = s
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let n = tmp[0];
    let k = tmp[1];

    s.clear();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut alist = s
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    alist.sort();

    let mut start = 0;
    let mut end = 1 << 30;
    let mut mid = (start + end) / 2;

    while start <= end {
        let mut res = 0;
        for i in 0..n {
            res += mid / alist[i as usize];
        }
        if res >= k {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
        mid = (start + end) / 2;
    }

    println!("{}", start);
}
