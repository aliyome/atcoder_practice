use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars
    };

    let mut t = 0;
    let mut a = 0;
    let mut win = '-';
    for i in 0..n {
        if s[i] == 'A' {
            a += 1;
            if a > t {
                win = 'A';
            }
        }
        if s[i] == 'T' {
            t += 1;
            if t > a {
                win = 'T';
            }
        }
    }

    println!("{}", win);
}
