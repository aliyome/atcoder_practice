use proconio::input;

fn main() {
    input! {
      n: usize,
      s: usize,
      a: [usize; n]
    }

    // dp[i][j] = i番目までの整数の中からいくつか選んで総和をjにすることができるか
    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;

    for i in 1..=n {
        for j in 1..=s {
            let a = a[i - 1];
            if 0 <= j as isize - a as isize && dp[i - 1][j - a] {
                dp[i][j] = true;
            }
        }
    }

    if dp[n][s] {
        println!("Yes");
    } else {
        println!("No");
    }
}
