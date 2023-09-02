use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h]
    };

    // 列ごとの連結成分
    // row[i][j] := i 行目の j 列目の連結成分の長さ
    let mut row = vec![vec![0; w]; h];
    for i in 0..h {
        let mut cnt = 0;
        for j in 0..w {
            if s[i][j] == '#' {
                for k in j - cnt..j {
                    row[i][k] = cnt;
                }
                cnt = 0;
            } else {
                cnt += 1;
            }
        }
        for k in w - cnt..w {
            row[i][k] = cnt;
        }
    }

    // 行ごとの連結成分
    // col[j][i] := j 列目の i 行目の連結成分の長さ
    let mut col = vec![vec![0; h]; w];
    for j in 0..w {
        let mut cnt = 0;
        for i in 0..h {
            if s[i][j] == '#' {
                for k in i - cnt..i {
                    col[j][k] = cnt;
                }
                cnt = 0;
            } else {
                cnt += 1;
            }
        }
        for k in h - cnt..h {
            col[j][k] = cnt;
        }
    }

    // 行と列の連結成分の話-1が答え
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if row[i][j] == 0 && col[j][i] == 0 {
                continue;
            }
            ans = ans.max(row[i][j] + col[j][i] - 1);
        }
    }

    println!("{}", ans);
}
