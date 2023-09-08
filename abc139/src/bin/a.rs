use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };

    let mut ans = 0;
    for i in 0..s.len() {
        if s[i] == t[i] {
            ans += 1;
        }
    }
    println!("{}", ans);
}
