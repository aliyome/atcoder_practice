use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    };

    s.sort();

    println!("{}", s.iter().map(|c| c.to_string()).collect::<String>());
}
