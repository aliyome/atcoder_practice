use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        (n, m, k, s, t, x): (usize, usize, usize, usize, usize, usize),
        uv: [(usize, usize); m],
    };

    // N頂点 M辺 単純無向グラフ
    // uv[i] := u - v を結ぶ辺
    // edges[u][v] := u - v を結ぶ辺が存在するか
    let mut edges = vec![vec![]; n + 1];
    for (u, v) in uv {
        edges[u].push(v);
        edges[v].push(u);
    }

    // a := [a0, a1, ..., ak] は何通りはあるか
    // - 1 <= ai <= N
    // a0 = s
    // ak = t
    // ai - ai+1 を結ぶ辺が存在する
    // a に x は偶数回(0でもいい)出現する

    // dp[i][j][e] := 末尾が j の時、i 文字目までの場合の数
    //                e:0 x が偶数回出現している, e:1 x が奇数回出現している
    let mut dp = vec![vec![vec![0; 2]; n + 1]; k + 2];
    if s == x {
        dp[0][s][1] = 1;
    } else {
        dp[0][s][0] = 1;
    }

    // i文字目まで見て
    for i in 0..=k {
        // j が末尾だったとき
        for j in 1..=n {
            // 次が k になるような辺があるなら
            for &k in &edges[j] {
                // 1文字前の通り数を足す
                if k == x {
                    // 次の文字が x なら偶奇を反転する
                    dp[i + 1][k][0] += dp[i][j][1];
                    dp[i + 1][k][1] += dp[i][j][0];
                    dp[i + 1][k][0] %= MOD;
                    dp[i + 1][k][1] %= MOD;
                } else {
                    dp[i + 1][k][0] += dp[i][j][0];
                    dp[i + 1][k][1] += dp[i][j][1];
                    dp[i + 1][k][0] %= MOD;
                    dp[i + 1][k][1] %= MOD;
                }
            }
        }
    }

    println!("{}", dp[k][t][0]);
}
