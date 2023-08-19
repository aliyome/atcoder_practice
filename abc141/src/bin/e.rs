use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars
    };

    // 10^6
    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            let t = s[i..j].iter().collect::<String>();
            if s[j..].iter().collect::<String>().contains(&t) {
                ans = ans.max(t.len());
            }
        }
    }

    println!("{}", ans);
}
