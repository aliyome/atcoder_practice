use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars
    };

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    for i in 0..n {
        if s[i] == 'A' {
            a += 1;
        } else if s[i] == 'B' {
            b += 1;
        } else {
            c += 1;
        }
        if a > 0 && b > 0 && c > 0 {
            println!("{}", i + 1);
            return;
        }
    }
}
