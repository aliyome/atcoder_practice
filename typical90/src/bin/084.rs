use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let mut rl = vec![];
    rl.push((s[0], 1));
    for i in 1..n {
        if s[i] == s[i - 1] {
            rl.last_mut().unwrap().1 += 1;
        } else {
            rl.push((s[i], 1));
        }
    }

    let total = n * (n + 1) / 2;

    // 余事象の数え上げ
    let mut x = 0;
    for (_, l) in rl {
        x += l * (l + 1) / 2;
    }

    println!("{}", total - x);
}
