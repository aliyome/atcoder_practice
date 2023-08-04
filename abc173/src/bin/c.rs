use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        c: [Chars; h]
    };

    let mut ans = 0;
    for i in 0..1 << h {
        let mut skip_row = vec![false; h];
        for k in 0..h {
            if i & 1 << k != 0 {
                skip_row[k] = true;
            }
        }
        for j in 0..1 << w {
            let mut skip_col = vec![false; w];
            for k in 0..w {
                if j & 1 << k != 0 {
                    skip_col[k] = true;
                }
            }

            let mut count = 0;
            for y in 0..h {
                if skip_row[y] {
                    continue;
                }
                for x in 0..w {
                    if skip_col[x] {
                        continue;
                    }
                    if c[y][x] == '#' {
                        count += 1;
                    }
                }
            }
            if count == k {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
