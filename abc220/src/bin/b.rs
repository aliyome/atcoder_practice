use proconio::{input, marker::Chars};

fn main() {
    input! {
        k: usize,
        mut a: Chars,
        mut b: Chars
    };

    a.reverse();
    b.reverse();

    let mut x = 0;
    for i in 0..a.len() {
        x += k.pow(i as u32) * a[i].to_digit(10).unwrap() as usize;
    }

    let mut y = 0;
    for i in 0..b.len() {
        y += k.pow(i as u32) * b[i].to_digit(10).unwrap() as usize;
    }

    println!("{}", x * y);
}
