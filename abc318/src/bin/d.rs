use proconio::input;

fn dfs(current_mask: usize, n: usize, edge: &Vec<Vec<usize>>, memo: &mut Vec<i64>) -> i64 {
    if current_mask == (1 << n) - 1 {
        return 0; // All vertices are visited
    }

    if memo[current_mask] != -1 {
        return memo[current_mask];
    }

    let mut max_weight = 0;
    for i in 0..n {
        // if vertex i is not visited yet
        if current_mask & (1 << i) == 0 {
            for j in i + 1..n {
                // if vertex j is also not visited yet
                if current_mask & (1 << j) == 0 {
                    let next_mask = current_mask | (1 << i) | (1 << j);
                    let weight = edge[i + 1][j + 1];
                    max_weight = max_weight.max(weight as i64 + dfs(next_mask, n, edge, memo));
                }
            }
        }
    }

    memo[current_mask] = max_weight;
    max_weight
}

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

    let mut memo = vec![-1; 1 << n];
    let result = dfs(0, n, &edge, &mut memo);

    println!("{}", result);
}
