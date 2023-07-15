use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    // 相性の悪い組み合わせをビットマスクで表す
    let mut bad = vec![];
    for &(a, b) in &ab {
        let mut mask = 0;
        mask |= 1 << (a - 1);
        mask |= 1 << (b - 1);
        bad.push(mask);
    }

    // 全組み合わせをビットマスクで表す
    let mut team = vec![];
    for i in 1..=1 << n {
        let mut mask = 0;
        for j in 0..n {
            if (i >> j) & 1 == 1 {
                mask |= 1 << j;
            }
        }
        team.push(mask);
    }

    for i in 0..t {
        for j in 0..(1 << n) {
            if dp[i][j] > 0 {
                for k in 0..n {
                    if (j >> k) & 1 == 0 && (j & bad[k]) == 0 {
                        dp[i + 1][j | (1 << k)] += dp[i][j];
                    }
                }
            }
        }
    }

    let mut ans = 0;
    for &x in &dp[t] {
        ans += x;
    }
    println!("{}", ans);
}
