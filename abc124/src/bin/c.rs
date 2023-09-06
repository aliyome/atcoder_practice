use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut a = 0;
    for i in 0..s.len() {
        if i % 2 == 0 && s[i] == '0' {
            a += 1;
        } else if i % 2 == 1 && s[i] == '1' {
            a += 1;
        }
    }

    let mut b = 0;
    for i in 0..s.len() {
        if i % 2 == 0 && s[i] == '1' {
            b += 1;
        } else if i % 2 == 1 && s[i] == '0' {
            b += 1;
        }
    }

    println!("{}", a.min(b));
}
