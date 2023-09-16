use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut ans = 0;
    for i in 0..s.len() {
        for j in i + 1..=s.len() {
            let a = s[i..j].iter().collect::<String>();
            let b = a.chars().rev().collect::<String>();
            if a == b {
                ans = ans.max(j - i);
            }
        }
    }
    println!("{}", ans);
}
