use proconio::{input, marker::Chars};

fn main() {
    input! {
      n: Chars,
    }

    let mut ans = 0;
    for (i, &c) in n.iter().rev().enumerate() {
        if c == '1' {
            ans += 2i64.pow(i as u32);
        }
    }

    println!("{}", ans);
}
