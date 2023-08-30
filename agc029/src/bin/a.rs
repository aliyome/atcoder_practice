use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut b = 0usize;
    let mut mov = 0usize;
    for i in 0..s.len() {
        if s[i] == 'B' {
            b += 1;
            mov += i;
        }
    }

    let mut x = 0usize;
    for i in s.len() - b..s.len() {
        x += i;
    }

    println!("{}", x - mov);
}
