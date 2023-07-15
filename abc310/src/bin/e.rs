use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut s: Chars,
    }

    let s: Vec<_> = s
        .into_iter()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let nand = |a: usize, b: usize| !(a & b) & 1;

    let mut ans = 0;
    for i in 0..n {
        let mut tmp = std::usize::MAX;
        for j in i..n {
            if i == j {
                tmp = s[i];
            } else {
                tmp = nand(tmp, s[j]);
            }
            ans += tmp;
        }
    }

    println!("{}", ans);
}
