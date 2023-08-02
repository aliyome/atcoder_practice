use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(usize, usize); n - 1],
        px: [(usize, usize); q],
    };

    // sum[i] := 頂点 i の部分木に加算する値
    let mut sum = vec![0; n + 1];
    for &(p, x) in &px {
        sum[p] += x;
    }

    // edge[i] := 頂点 i から伸びる辺の先
    let mut edge = vec![vec![]; n + 1];
    for &(a, b) in &ab {
        edge[a].push(b);
        edge[b].push(a);
    }

    // 親から累積和をdfsで計算する
    let mut visited = vec![false; n + 1];
    visited[1] = true;
    dfs(1, &edge, &mut sum, &mut visited);

    // 出力
    for i in 1..=n {
        print!("{} ", sum[i]);
    }
}

fn dfs(v: usize, edge: &Vec<Vec<usize>>, sum: &mut Vec<usize>, visited: &mut Vec<bool>) {
    for &next in &edge[v] {
        if visited[next] {
            continue;
        }
        visited[next] = true;
        sum[next] += sum[v];
        dfs(next, edge, sum, visited);
    }
}
