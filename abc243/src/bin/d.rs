use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        mut x: usize,
        s: Chars
    };

    for c in s {
        match c {
            'U' => x /= 2,
            'L' => x *= 2,
            'R' => x = x * 2 + 1,
            _ => unreachable!(),
        }
    }

    println!("{}", x);
}
