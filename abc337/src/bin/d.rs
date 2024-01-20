use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        grid: [Chars; h],
    }

    let mut ans = std::usize::MAX;

    // 水平方向の累積和
    for i in 0..h {
        let mut h_cum = vec![(0, 0, 0); w + 1];
        for j in 0..w {
            let c = grid[i][j];
            let j = j + 1;
            h_cum[j] = h_cum[j - 1];

            if c == '.' {
                h_cum[j].0 += 1;
            } else if c == 'o' {
                h_cum[j].1 += 1;
            } else if c == 'x' {
                h_cum[j].2 += 1;
            }
        }
        if w >= k {
            for j in 0..=w - k {
                let diff = (
                    h_cum[j + k].0 - h_cum[j].0,
                    h_cum[j + k].1 - h_cum[j].1,
                    h_cum[j + k].2 - h_cum[j].2,
                );
                if (diff.0 + diff.1) >= k && diff.2 == 0 {
                    ans = ans.min(k - diff.1);
                }
            }
        }
    }

    // 垂直方向の累積和
    for j in 0..w {
        let mut v_cum = vec![(0, 0, 0); h + 1];
        for i in 0..h {
            let c = grid[i][j];
            let i = i + 1;
            v_cum[i] = v_cum[i - 1];

            if c == '.' {
                v_cum[i].0 += 1;
            } else if c == 'o' {
                v_cum[i].1 += 1;
            } else if c == 'x' {
                v_cum[i].2 += 1;
            }
        }
        if h >= k {
            for i in 0..=h - k {
                let diff = (
                    v_cum[i + k].0 - v_cum[i].0,
                    v_cum[i + k].1 - v_cum[i].1,
                    v_cum[i + k].2 - v_cum[i].2,
                );
                if (diff.0 + diff.1) >= k && diff.2 == 0 {
                    ans = ans.min(k - diff.1);
                }
            }
        }
    }

    if ans == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
