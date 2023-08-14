use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut t = s.clone();
    for i in 1..=s.len() / 2 {
        t.swap(2 * i - 1 - 1, 2 * i - 1);
    }

    println!("{}", t.iter().collect::<String>());
}
