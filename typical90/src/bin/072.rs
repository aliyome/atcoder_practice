use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize, w: usize,
        c: [Chars; h]
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == '#' {
                continue;
            }
            dfs(i, j, i, j, 0, &mut vec![vec![false; w]; h], &mut ans, &c);
        }
    }

    if ans < 3 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

fn dfs(
    si: usize,
    sj: usize,
    i: usize,
    j: usize,
    sum: usize,
    visited: &mut Vec<Vec<bool>>,
    ans: &mut usize,
    c: &Vec<Vec<char>>,
) {
    if visited[si][sj] {
        *ans = (*ans).max(sum);
        return;
    };

    if i > 0 && !visited[i - 1][j] && c[i - 1][j] == '.' {
        visited[i - 1][j] = true;
        dfs(si, sj, i - 1, j, sum + 1, visited, ans, c);
        visited[i - 1][j] = false;
    }
    if i < visited.len() - 1 && !visited[i + 1][j] && c[i + 1][j] == '.' {
        visited[i + 1][j] = true;
        dfs(si, sj, i + 1, j, sum + 1, visited, ans, c);
        visited[i + 1][j] = false;
    }
    if j > 0 && !visited[i][j - 1] && c[i][j - 1] == '.' {
        visited[i][j - 1] = true;
        dfs(si, sj, i, j - 1, sum + 1, visited, ans, c);
        visited[i][j - 1] = false;
    }
    if j < visited[0].len() - 1 && !visited[i][j + 1] && c[i][j + 1] == '.' {
        visited[i][j + 1] = true;
        dfs(si, sj, i, j + 1, sum + 1, visited, ans, c);
        visited[i][j + 1] = false;
    }
}
