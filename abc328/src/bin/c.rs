use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        lr: [(usize, usize); q],
    }

    // 累積和
    let mut sum = vec![0; n + 1];
    for i in 1..n {
        sum[i + 1] = sum[i] + if s[i - 1] == s[i] { 1 } else { 0 };
    }

    for (l, r) in lr {
        println!("{}", sum[r] - sum[l]);
    }
}
