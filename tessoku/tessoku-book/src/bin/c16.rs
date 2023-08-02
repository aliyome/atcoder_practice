use proconio::input;

fn main() {
    input! {
      n: usize,
      m: usize,
      k: usize,
      mut asbt: [(usize, usize, usize, usize); m]
    }
    // 到着時刻に乗り換え時間を足す
    for i in 0..m {
        asbt[i].3 += k;
    }

    // 区間スケジューリングは貪欲法で解けるので到着が早い順にソートする
    asbt.sort_by(|a, b| a.3.cmp(&b.3));

    // グラフを作る
    // NOTE: 逆向きに辺を張る
    let mut graph = vec![vec![]; n + 2];
    for &(a, b, s, t) in &asbt {
        graph[b].push((a, s, t));
    }

    // グラフに始点と終点を追加
    for i in 1..=n {
        graph[i].push((0, 0, 0));
        graph[n + 1].push((i, 0, 0));
    }

    // dp[i][j] := i番目のフライトまで見た時、最後に空港jに居るときの最大フライト回数
    let mut dp = vec![0; m + 1];
    for i in 1..=m {
        for &(a, s, t) in &graph[i] {
            dp[i][t] = dp[i][t].max(dp[a][s] + 1);
        }
    }

    println!("{}", dp[m][n]);
}
