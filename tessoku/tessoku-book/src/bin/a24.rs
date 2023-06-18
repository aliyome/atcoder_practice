use proconio::input;

fn main() {
    input! {
      n: usize, // <= 10^5
      a: [usize; n] // a[i] <= 5*10^5
    }

    // dp[i] := ai が最後の要素となる最長の増加部分列の長さ
    let mut dp = vec![0usize; n];
    for i in 0..n {
        dp[i] = 1;
        for j in 0..i {
            if a[j] < a[i] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
    }

    println!("{:?}", dp[n - 1]);
}
