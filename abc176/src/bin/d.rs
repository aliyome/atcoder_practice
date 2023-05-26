use proconio::{input, marker::Chars};

// 01-BFS

fn main() {
    input! {
        (h, w): (usize, usize),
        (ch, cw): (usize, usize),
        (dh, dw): (usize, usize),
        s: [Chars; h]
    };

    // dp[現在のy座標][現在のx座標] = 到達コスト
    let mut dp = vec![vec![std::i64::MAX; w]; h];
    let mut queue = std::collections::VecDeque::new();

    dp[ch - 1][cw - 1] = 0;
    queue.push_back((ch as i64 - 1, cw as i64 - 1));

    while let Some((y, x)) = queue.pop_front() {
        let dist = dp[y as usize][x as usize];
        // コスト0で到達可能な範囲をキューに 前から 追加
        for (vy, vx) in [(0, 1), (0, -1i64), (1, 0), (-1i64, 0)].iter() {
            let (ny, nx) = (y + vy, x + vx);
            if ny >= 0
                && nx >= 0
                && ny < h as i64
                && nx < w as i64
                && s[ny as usize][nx as usize] == '.'
                && dp[ny as usize][nx as usize] > dist
            {
                queue.push_front((ny, nx));
                dp[ny as usize][nx as usize] = dist;
            }
        }
        // コスト1で到達可能な範囲をキューに 後ろから 追加
        for vy in -2i64..=2 {
            for vx in -2i64..=2 {
                let (ny, nx) = (y + vy, x + vx);
                if ny >= 0
                    && nx >= 0
                    && ny < h as i64
                    && nx < w as i64
                    && s[ny as usize][nx as usize] == '.'
                    && dp[ny as usize][nx as usize] > dist
                {
                    queue.push_back((ny, nx));
                    dp[ny as usize][nx as usize] = dist + 1;
                }
            }
        }
    }
    if dp[dh - 1][dw - 1] == std::i64::MAX {
        println!("-1");
        return;
    }
    println!("{}", dp[dh - 1][dw - 1]);
}
