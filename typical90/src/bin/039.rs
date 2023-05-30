use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1]
    };

    // 無向グラフ
    let mut graph = vec![vec![]; n + 1];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    // dp[x] = 各頂点の、その頂点を含む子孫の数
    let mut dp = vec![0; n + 1];

    // DFS
    dfs(1, 0, &graph, &mut dp);

    // 答え
    let mut ans = 0;
    for &(a, b) in &ab {
        let r = dp[a].min(dp[b]);
        ans += r * (n - r);
    }

    println!("{}", ans);
}

fn dfs(pos: usize, pre: usize, graph: &Vec<Vec<usize>>, dp: &mut Vec<usize>) {
    // 自身は1
    dp[pos] = 1;
    for &next in &graph[pos] {
        // 逆転防止
        if next == pre {
            continue;
        }
        dfs(next, pos, &graph, dp);
        // 帰りがけに子孫の数を足す
        dp[pos] += dp[next];
    }
}
