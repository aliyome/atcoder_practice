use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h]
    };

    let mut dist = vec![vec![std::usize::MAX; w]; h];
    let mut queue = std::collections::VecDeque::new();
    dist[0][0] = 0;
    queue.push_back((0, 0, 0));
    while let Some((i, j, d)) = queue.pop_front() {
        if dist[i][j] < d {
            continue;
        }
        if j + 1 < w && c[i][j + 1] == '.' && dist[i][j + 1] > d + 1 {
            dist[i][j + 1] = d + 1;
            queue.push_back((i, j + 1, d + 1));
        }
        if i + 1 < h && c[i + 1][j] == '.' && dist[i + 1][j] > d + 1 {
            dist[i + 1][j] = d + 1;
            queue.push_back((i + 1, j, d + 1));
        }
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if dist[i][j] != std::usize::MAX {
                ans = ans.max(dist[i][j]);
            }
        }
    }

    println!("{}", ans + 1);
}
