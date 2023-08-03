use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize
    }

    let mut count = 0;
    let mut led = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            if led[i][j] == 0 {
                led[i][j] = 1;
                count += 1;
                if h == 1 || w == 1 {
                    continue;
                }

                if i + 1 < h {
                    led[i + 1][j] = 2;
                }
                if j + 1 < w {
                    led[i][j + 1] = 2;
                }
                if i + 1 < h && j + 1 < w {
                    led[i + 1][j + 1] = 2;
                }
            }
        }
    }

    println!("{}", count);
}
