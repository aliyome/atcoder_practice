use proconio::input;
use superslice::Ext;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
      n: usize, // <= 1.5 x 10^5
      w: usize, // <= 10^9
      l: usize,
      r: usize,
      mut x: [usize; n] // x <= 10^9
    }

    // 西岸を0、東岸をwとする
    x.insert(0, 0);
    x.push(w);

    // 配るDP
    // dp[i] := i番目の足場に来る方法の数
    let mut dp = vec![0usize; n + 2];
    let mut imos = vec![0usize; n + 2];
    dp[0] = 1;
    for i in 0..=n + 1 {
        if i > 0 {
            // 累積和
            imos[i] = (imos[i] + imos[i - 1]) % MOD;
            dp[i] = imos[i];
        }

        // O(NlogN)になるように、二分探索を使う
        let il = x.lower_bound(&(x[i] + l));
        let ir = x.upper_bound(&(x[i] + r));
        println!("{} {} {}", i, il, ir);
        // いもす法で累積和を求める
        if il < n + 2 {
            imos[il] = (imos[il] + dp[i]) % MOD;
        }
        if ir < n + 2 {
            imos[ir] = (imos[ir] + MOD - dp[i]) % MOD;
        }
    }

    println!("{}", dp[n + 1]);
}
