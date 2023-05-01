use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars
    };

    if n % 2 > 0 {
        println!("No");
        return;
    }

    let x = &s[0..n / 2];
    let y = &s[n / 2..];

    if x == y {
        println!("Yes");
    } else {
        println!("No");
    }
}
