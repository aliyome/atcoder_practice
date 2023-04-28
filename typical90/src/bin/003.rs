use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [[usize; 2]; n as usize - 1]
    };

    // edges に隣接リストを作る
    let mut edges = vec![vec![]; n];
    for e in ab {
        edges[e[0] - 1].push(e[1] - 1);
        edges[e[1] - 1].push(e[0] - 1);
    }

    // dist に各頂点の深さを入れる
    let mut dist = vec![0; n];

    // 0 番目の頂点から dfs をする
    dfs(&edges, 0, n, &mut dist);

    // 0 番目の頂点から最も遠い頂点を探す
    let mut max_dist = 0;
    let mut max_dist_idx = 0;
    for i in 0..n {
        if dist[i] > max_dist {
            max_dist = dist[i];
            max_dist_idx = i;
        }
    }

    // dist に各頂点の深さを入れる
    let mut dist = vec![0; n];

    // 最も遠い頂点から dfs をする
    dfs(&edges, max_dist_idx, n, &mut dist);

    // 最も遠い頂点から最も遠い頂点を探す
    let mut max_dist = 0;
    let mut max_dist_idx = 0;
    for i in 0..n {
        if dist[i] > max_dist {
            max_dist = dist[i];
            max_dist_idx = i;
        }
    }

    // 答えは最も遠い頂点から最も遠い頂点の距離 + 1
    println!("{}", max_dist + 1);
}

fn dfs(edges: &Vec<Vec<usize>>, curr: usize, prev: usize, dist: &mut Vec<usize>) {
    for next in edges[curr].iter() {
        if *next == prev {
            continue;
        }
        dist[*next] = dist[curr] + 1;
        dfs(edges, *next, curr, dist);
    }
}
