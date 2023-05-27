use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        mut t: Chars
    };

    for i in 0..n {
        if s[i] == '1' {
            s[i] = 'l';
        }
        if t[i] == '1' {
            t[i] = 'l';
        }
        if s[i] == '0' {
            s[i] = 'o';
        }
        if t[i] == '0' {
            t[i] = 'o';
        }
    }

    if s == t {
        println!("Yes");
    } else {
        println!("No");
    }
}
