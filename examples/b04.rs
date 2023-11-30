use std::io;

fn main() {
    let n = input_line().trim().to_string();

    let mut result = 0;
    for (i, c) in n.chars().rev().enumerate() {
        if c == '1' {
            result += 1 << i;
        }
    }

    println!("{result}");
}

fn input_line() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s
}
