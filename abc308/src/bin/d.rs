use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

// const INF: usize = std::usize::MAX;
const INF: usize = 1000;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let dx = [1, 0, -1, 0];
    let dy = [0, 1, 0, -1];
    let snuke = "snuke".chars().collect::<Vec<char>>();

    let mut dist = vec![vec![INF; w]; h];
    let mut que = VecDeque::new();

    dist[0][0] = 1;
    que.push_back((0, 0));

    while let Some((x, y)) = que.pop_front() {
        for dir in 0..4 {
            let nx = x as isize + dx[dir];
            let ny = y as isize + dy[dir];
            if nx < 0 || nx >= h as isize || ny < 0 || ny >= w as isize {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if s[nx][ny] == snuke[dist[x][y] % 5] && dist[nx][ny] == INF {
                dist[nx][ny] = dist[x][y] + 1;
                que.push_back((nx, ny));
            }
        }
    }

    // for i in 0..h {
    //     println!("{:?}", dist[i]);
    // }

    if dist[h - 1][w - 1] != INF {
        println!("Yes");
    } else {
        println!("No");
    }
}
