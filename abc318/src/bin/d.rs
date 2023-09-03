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
    dfs(&edge, &mut vec![false; n + 1], 0, n, 0, &mut ans);

    println!("{}", ans);
}

fn dfs(
    edge: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    v: usize,
    n: usize,
    sum: usize,
    ans: &mut usize,
) {
    if v + 2 > n {
        *ans = (*ans).max(sum);
        return;
    }
    for i in 1..=n {
        if visited[i] {
            continue;
        }
        visited[i] = true;

        for j in i + 1..=n {
            if visited[j] {
                continue;
            }
            visited[j] = true;

            let sum = sum + edge[i][j];
            dfs(edge, visited, v + 2, n, sum, ans);

            visited[j] = false;
        }

        visited[i] = false;
    }
}
