fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let nx = s
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let n = nx[0];
    let x = nx[1];

    s.clear();
    std::io::stdin().read_line(&mut s).unwrap();
    let alist = s
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut start = 0;
    let mut end = n - 1;
    let mut mid = (start + end) / 2;

    while start <= end {
        match alist[mid as usize].cmp(&x) {
            std::cmp::Ordering::Equal => {
                println!("{}", mid + 1);
                return;
            }
            std::cmp::Ordering::Less => start = mid + 1,
            std::cmp::Ordering::Greater => end = mid - 1,
        }
        mid = (start + end) / 2;
    }
}
