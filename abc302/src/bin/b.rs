use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let directions = [
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
    ];
    let snuke = ['s', 'n', 'u', 'k', 'e'];
    for i in 0..h {
        for j in 0..w {
            // 1文字目 's' を探す
            if s[i][j] == snuke[0] {
                // 8方向に探す
                for &(vy, vx) in &directions {
                    let mut y = i as i64;
                    let mut x = j as i64;
                    let mut ans = vec![];
                    ans.push((y, x));
                    // 2文字目以降を探す
                    for k in 1..snuke.len() {
                        y = y + vy;
                        x = x + vx;
                        if x < 0 || y < 0 || x >= w as i64 || y >= h as i64 {
                            break;
                        }
                        if s[y as usize][x as usize] == snuke[k] {
                            ans.push((y, x));
                        } else {
                            break;
                        }
                    }
                    if ans.len() == snuke.len() {
                        for &(y, x) in &ans {
                            println!("{} {}", y + 1, x + 1);
                        }
                        return;
                    }
                }
            }
        }
    }
}
