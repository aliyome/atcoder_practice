use proconio::input;

// 区間DP
fn main() {
    input! {
        n: usize,
       a: [i64; n * 2]
    }

    // dp[l][r] := 区間[l, r)の最小コスト
    // とりあえずデカい値で埋める。 r < l はありえないので埋めなくても良い。
    let mut dp = vec![vec![10i64.pow(9); n * 2]; n * 2];

    // 区間の長さが2の場合は2つの要素の差の絶対値が最小コストとなる。他に方法が無いため。
    for l in 0..n * 2 - 1 {
        dp[l][l + 1] = (a[l] - a[l + 1]).abs();
    }

    // DP
    for k in (1..n * 2).step_by(2) {
        let mut l = 0;
        let mut r = k;
        while l < n * 2 && r < n * 2 {
            // 1. 先に間を取り除いてから両端を取り除く
            //    (l, r) と [l, r]
            dp[l][r] = dp[l][r].min(dp[l + 1][r - 1] + (a[l] - a[r]).abs());
            // 2. 2つの区間に分ける
            //    [l, l+i) と (l+i+1, r]
            for i in (1..k).step_by(2) {
                dp[l][r] = dp[l][r].min(dp[l][l + i] + dp[l + i + 1][r]);
            }
            l += 1;
            r += 1;
        }
    }

    println!("{}", dp[0][n * 2 - 1]);
}
