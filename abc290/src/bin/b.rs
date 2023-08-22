use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut k: usize,
        s: Chars
    };

    for i in 0..n {
        if k == 0 {
            print!("x");
        } else if s[i] == 'o' {
            k -= 1;
            print!("o");
        } else {
            print!("x");
        }
    }
}
