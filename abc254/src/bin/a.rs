use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    };

    println!("{}", n[n.len() - 2..].into_iter().collect::<String>());
}
