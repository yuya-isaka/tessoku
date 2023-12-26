fn lower_bound<T: Ord>(list: &[T], x: &T) -> usize {
    let mut start = 0;
    let mut end = list.len();

    while start < end {
        let mid = start + (end - start) / 2;

        if list[mid] < *x {
            start = mid + 1;
        } else {
            end = mid;
        }
    }

    start
}

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let tmp = s
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let n = tmp[0];
    let x = tmp[1];
    let alist = s
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let ruisekiwa = alist
        .iter()
        .scan(0, |state, &x| {
            *state += x;
            Some(*state)
        })
        .collect::<Vec<_>>();

    let mut ans = 0;
    for i in 0..n {
        let mut target = x;
        if i >= 1 {
            target += ruisekiwa[i as usize - 1];
        }
        let index = lower_bound(&ruisekiwa, &target);
        ans += n - index as u64;
    }
    println!("{}", ans);
}
