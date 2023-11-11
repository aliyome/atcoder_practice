use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        lr: [(usize, usize); q],
    }

    // 累積和を計算するための配列を準備
    let mut cum_sum = vec![0; n + 1];

    // 文字列を一回走査し、隣接する同じ文字の数をカウント
    for i in 0..n - 1 {
        cum_sum[i + 1] = cum_sum[i] + if s[i] == s[i + 1] { 1 } else { 0 };
    }

    // 累積和を用いてクエリの答えを計算
    for (l, r) in lr {
        let ans = cum_sum[r - 1] - cum_sum[l - 1];
        println!("{}", ans);
    }
}
