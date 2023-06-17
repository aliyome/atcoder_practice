use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars
    };

    let mut t = vec![];
    for i in 0..n {
        t.push(s[i]);
        t.push(s[i]);
    }

    println!("{}", t.iter().collect::<String>());
}
