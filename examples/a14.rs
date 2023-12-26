fn main() {
    fn input() -> Vec<u64> {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>()
    }
    let tmp = input();
    let n = tmp[0];
    let k = tmp[1];

    let alist = input();
    let blist = input();
    let clist = input();
    let dlist = input();

    let mut cd = vec![];
    for i in 0..n {
        for j in 0..n {
            cd.push(clist[i as usize] + dlist[j as usize]);
        }
    }
    cd.sort();
    cd.dedup();

    fn binary_search<T: Ord>(list: &[T], x: &T) -> Option<usize> {
        let mut start = 0;
        let mut end = list.len();
        while start < end {
            let mid = start + (end - start) / 2;
            match list[mid].cmp(x) {
                std::cmp::Ordering::Equal => return Some(mid),
                std::cmp::Ordering::Less => start = mid + 1,
                std::cmp::Ordering::Greater => end = mid,
            }
        }
        None
    }

    for i in 0..n {
        for j in 0..n {
            let target = k - alist[i as usize] - blist[j as usize];
            if binary_search(&cd, &target).is_some() {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
