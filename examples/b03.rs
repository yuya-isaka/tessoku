use itertools::Itertools;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    std::io::stdin().read_line(&mut s).unwrap();
    let a: Vec<u32> = s
        .split_whitespace()
        .map(|tmp| tmp.parse().unwrap())
        .collect();

    let flag = a
        .iter()
        .tuple_combinations()
        .any(|(x, y, z)| x + y + z == 1000);
    println!("{}", if flag { "Yes" } else { "No" });
}
