use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut rl = vec![s[0]];
    let mut prev = s[0];
    for i in 1..s.len() {
        if prev != s[i] {
            rl.push(s[i]);
            prev = s[i];
        }
    }

    println!("{}", rl.len() - 1);
}
