use proconio::input;

// 021 - Come Back in One Piece（★5）
// https://atcoder.jp/contests/typical90/tasks/typical90_u
fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(usize, usize); m]
    }

    let mut graph = vec![vec![]; n + 1];
    let mut graph_inverted = vec![vec![]; n + 1];
    for (a, b) in ab {
        graph[a].push(b);
        graph_inverted[b].push(a);
    }

    // 強連結成分分解を行う

    // 1. DFS で帰りがけ順に番号を記録する
    let mut used = vec![false; n + 1];
    let mut order = vec![];
    for i in 1..=n {
        if !used[i] {
            dfs(i, &graph, &mut used, &mut order);
        }
    }

    // 2. 辺の向きをすべて反転させ、番号の大きい順に DFS する
    order.reverse();
    let mut used = vec![false; n + 1];
    let mut scc_list = vec![];
    for &i in &order {
        if used[i] {
            continue;
        }
        let mut scc = vec![];
        dfs2(i, &graph_inverted, &mut used, &mut scc);
        scc_list.push(scc);
    }

    // 3. 頂点数が2以上の強連結成分の数を数える
    let mut ans = 0;
    for scc in scc_list {
        let n = scc.len();
        if n < 2 {
            continue;
        }
        let nc2 = n * (n - 1) / 2;
        ans += nc2;
    }

    println!("{}", ans);
}

// dfs で帰りがけ順に番号を記録する
fn dfs(v: usize, graph: &Vec<Vec<usize>>, used: &mut Vec<bool>, order: &mut Vec<usize>) {
    used[v] = true;
    for &next in &graph[v] {
        if !used[next] {
            dfs(next, graph, used, order);
        }
    }
    // dfs でもう辿れる頂点がなくなったら番号を記録する
    order.push(v);
}

// dfs で行き止まりにあたるまでの頂点を記録する
fn dfs2(v: usize, graph: &Vec<Vec<usize>>, used: &mut Vec<bool>, scc: &mut Vec<usize>) {
    used[v] = true;
    scc.push(v);
    for &next in &graph[v] {
        if !used[next] {
            dfs2(next, graph, used, scc);
        }
    }
}
