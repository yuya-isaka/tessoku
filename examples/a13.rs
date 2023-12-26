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
    let alist = s
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    fn upper_bound<T: Ord>(slice: &[T], value: &T) -> usize {
        let mut low = 0;
        let mut high = slice.len();

        while low < high {
            let mid = low + (high - low) / 2;
            if slice[mid] <= *value {
                low = mid + 1;
            } else {
                high = mid;
            }
        }

        low
    }

    let mut ans = 0;
    for i in 0..n {
        let target = k + alist[i as usize];
        let tmp = upper_bound(&alist, &target);
        ans += n - tmp as u64;
    }

    println!("{}", n * (n - 1) / 2 - ans);
}
