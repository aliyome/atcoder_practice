use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let dx = [-1, 0, 1, -1, 1, -1, 0, 1];
    let dy = [-1, -1, -1, 0, 0, 1, 1, 1];
    let snuke = vec!['s', 'n', 'u', 'k', 'e'];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 's' {
                'outer: for dir in 0..8 {
                    let mut v = vec![(i + 1, j + 1)];
                    let mut x = i;
                    let mut y = j;
                    for _ in 0..4 {
                        x = (x as isize + dx[dir]) as usize;
                        y = (y as isize + dy[dir]) as usize;
                        if x < h && y < w && s[x][y] == snuke[v.len()] {
                            v.push((x + 1, y + 1));
                        } else {
                            continue 'outer;
                        }
                    }
                    if v.len() == 5 {
                        for (x, y) in v {
                            println!("{} {}", x, y);
                        }
                        return;
                    }
                }
            }
        }
    }
}
