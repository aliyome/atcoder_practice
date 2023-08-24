use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        (h, w): (usize, usize),
        (rs, cs): (usize, usize),
        (rt, ct): (usize, usize),
        s: [Chars; h]
    }
    let rs = rs - 1;
    let cs = cs - 1;
    let rt = rt - 1;
    let ct = ct - 1;

    let mut dp = vec![vec![std::usize::MAX; w]; h];
    dp[rs][cs] = 0;

    // 方向: 0: ↑, 1: →, 2: ↓, 3: ←
    let mut queue = VecDeque::new();
    queue.push_back((rs, cs, 0, 0));
    queue.push_back((rs, cs, 1, 0));
    queue.push_back((rs, cs, 2, 0));
    queue.push_back((rs, cs, 3, 0));

    while let Some((r, c, dir, cnt)) = queue.pop_front() {
        // d: 進行方向
        for d in 0..4 {
            let mut next_r = r;
            let mut next_c = c;

            let next_cnt = if dir == d { cnt } else { cnt + 1 };

            if d == 0 && r >= 1 {
                next_r = r - 1;
            } else if d == 1 && c + 1 < w {
                next_c = c + 1;
            } else if d == 2 && r + 1 < h {
                next_r = r + 1;
            } else if d == 3 && c >= 1 {
                next_c = c - 1;
            } else {
                continue;
            }

            if s[next_r][next_c] == '.' && dp[next_r][next_c] >= next_cnt {
                dp[next_r][next_c] = next_cnt;
                if dir == d {
                    queue.push_front((next_r, next_c, d, next_cnt));
                } else {
                    queue.push_back((next_r, next_c, d, next_cnt));
                }
            }
        }
    }

    println!("{}", dp[rt][ct]);
}
