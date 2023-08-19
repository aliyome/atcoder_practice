use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        l: usize,
        s: Chars
    };

    let mut ans = 0;
    let mut tab = 1;
    for &c in &s {
        if c == '+' {
            tab += 1;
        } else {
            tab -= 1;
        }
        if tab > l {
            ans += 1;
            tab = 1;
        }
    }
    println!("{}", ans);
}
