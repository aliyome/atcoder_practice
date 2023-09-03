use std::vec;

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    // edge[i][j]は頂点iと頂点jの間の距離
    let mut edge = vec![vec![0; n + 1]; n + 1];
    for i in 1..n {
        input! {
            d: [usize; n - i]
        }
        for j in i + 1..=n {
            edge[i][j] = d[j - i - 1];
            edge[j][i] = d[j - i - 1];
        }
    }

    let mut ans = 0;
    if n % 2 == 0 {
        dfs(&edge, &mut vec![false; n + 1], n, 0, &mut ans);
    } else {
        for i in 1..=n {
            let mut visited = vec![false; n + 1];
            visited[i] = true;
            dfs(&edge, &mut visited, n, 0, &mut ans);
        }
    }

    println!("{}", ans);
}

fn dfs(edge: &Vec<Vec<usize>>, visited: &mut Vec<bool>, n: usize, sum: usize, ans: &mut usize) {
    // 全頂点を訪れたら終了
    if visited.iter().skip(1).all(|&x| x) {
        *ans = (*ans).max(sum);
        return;
    }
    // 訪れていない一番若い頂点iを探す
    let mut i = 0;
    for ii in 1..=n {
        if visited[ii] {
            continue;
        }
        i = ii;
        break;
    }
    visited[i] = true;

    // ペアとなる頂点j
    for j in i + 1..=n {
        if visited[j] {
            continue;
        }
        visited[j] = true;

        // コスト加算
        let sum = sum + edge[i][j];
        dfs(edge, visited, n, sum, ans);

        visited[j] = false;
    }

    visited[i] = false;
}
