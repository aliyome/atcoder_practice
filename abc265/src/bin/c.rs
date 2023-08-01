use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        g: [Chars; h]
    };

    let mut stack = vec![];
    let mut visited = vec![vec![false; w]; h];
    stack.push((0, 0));
    visited[0][0] = true;

    let mut last = (0, 0);
    while let Some((i, j)) = stack.pop() {
        last = (i, j);
        if g[i][j] == 'U' && i >= 1 {
            if !visited[i - 1][j] {
                stack.push((i - 1, j));
                visited[i - 1][j] = true;
            } else {
                println!("-1");
                return;
            }
        } else if g[i][j] == 'D' && i + 1 < h {
            if !visited[i + 1][j] {
                stack.push((i + 1, j));
                visited[i + 1][j] = true;
            } else {
                println!("-1");
                return;
            }
        } else if g[i][j] == 'L' && j >= 1 {
            if !visited[i][j - 1] {
                stack.push((i, j - 1));
                visited[i][j - 1] = true;
            } else {
                println!("-1");
                return;
            }
        } else if g[i][j] == 'R' && j + 1 < w {
            if !visited[i][j + 1] {
                stack.push((i, j + 1));
                visited[i][j + 1] = true;
            } else {
                println!("-1");
                return;
            }
        }
    }
    println!("{} {}", last.0 + 1, last.1 + 1);
}
