use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [usize; n]
    };

    let total = t.iter().sum::<usize>();

    // dp[i][j] := i品目までを使ってj分で全部の料理を作れるか
    let mut dp = vec![vec![0; 100_001]; n + 1];
    dp[0][0] = 1;
    for i in 0..n {
        dp[i + 1] = dp[i].clone();
        let t = t[i];
        for j in 0..100_000 {
            if dp[i][j] == 0 {
                continue;
            }
            // i品目までを何分で作れるか
            dp[i + 1][j + t] = 1;
        }
    }

    let mut ans = std::usize::MAX;
    for a in 0..=100_000 {
        if dp[n][a] == 1 {
            let b = total - a;
            let duration = a.max(b);
            ans = ans.min(duration);
        }
    }

    println!("{}", ans);
}
