use itertools::Itertools;

fn solve() {
    let input_string: Vec<String> = input_array();
    let _n: u32 = input_string.get(0).unwrap().parse().unwrap();

    let a_list: Vec<u64> = input_array().iter().map(|s| s.parse().unwrap()).collect();

    // for i in 0..n {
    //     for j in (i + 1)..n {
    //         for k in (j + 1)..n {
    //             if a_list[i as usize] + a_list[j as usize] + a_list[k as usize] == 1000 {
    //                 println!("Yes");
    //                 return;
    //             }
    //         }
    //     }
    // }
    // println!("No");

    let flag = a_list
        .iter()
        .tuple_combinations()
        .any(|(i, j, k)| i + j + k == 1000);

    println!("{}", if flag { "Yes" } else { "No" });
}

fn main() {
    std::thread::Builder::new()
        .name("Big Stack".into())
        .stack_size(32 * 1024 * 1024) // 32MBスタック
        .spawn(solve)
        .unwrap()
        .join()
        .unwrap();
}

fn input_array() -> Vec<String> {
    let mut inputed_number: String = String::new();
    std::io::stdin().read_line(&mut inputed_number).unwrap();
    return inputed_number
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
}
