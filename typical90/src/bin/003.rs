use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [[usize; 2]; n as usize - 1]
    };

    // 辺の情報を格納する配列 edges[from][to] = bool
    let mut edges = vec![vec![false; n + 1]; n + 1];
    for edge in ab {
        let from = edge[0];
        let to = edge[1];
        // 木なので双方向に辺を張る
        edges[from][to] = true;
        edges[to][from] = true;
    }

    // 木の直径を求める
    // 頂点1から最も遠い頂点を求める
    let mut dist = vec![0; n + 1];
    let mut dist2 = vec![0; n + 1];
    dfs(&mut edges.clone(), 1, &mut dist, n, 1);
    let mut max = 0;
    let mut max_idx = 0;
    for i in 1..n + 1 {
        if dist[i] > max {
            max = dist[i];
            max_idx = i;
        }
    }
    // println!("{:?}", dist);
    // 最も遠い頂点からもう一度最も遠い頂点を求める
    dfs(&mut edges.clone(), max_idx, &mut dist2, n, 1);
    max = 0;
    for i in 1..n + 1 {
        if dist2[i] > max {
            max = dist2[i];
        }
    }
    // println!("{:?}", dist2);
    println!("{}", max + 1);
}

fn dfs(edges: &mut Vec<Vec<bool>>, cur: usize, dist: &mut Vec<usize>, n: usize, d: usize) {
    for i in 1..n + 1 {
        if edges[cur][i] {
            edges[cur][i] = false;
            edges[i][cur] = false;
            dist[i] += d;
            dfs(edges, i, dist, n, d + 1);
        }
    }
}
