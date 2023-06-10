use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut min_i = h;
    let mut max_i = 0;
    let mut min_j = w;
    let mut max_j = 0;

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                min_i = min_i.min(i);
                max_i = max_i.max(i);
                min_j = min_j.min(j);
                max_j = max_j.max(j);
            }
        }
    }

    for i in min_i..=max_i {
        for j in min_j..=max_j {
            if s[i][j] == '.' {
                println!("{} {}", i + 1, j + 1);
                return;
            }
        }
    }
}
