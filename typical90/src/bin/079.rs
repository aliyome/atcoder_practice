use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut a: [[isize; w]; h],
        b: [[isize; w]; h]
    }

    let mut ans = 0;
    for i in 0..h - 1 {
        for j in 0..w - 1 {
            let diff = b[i][j] - a[i][j];
            if diff == 0 {
                continue;
            }
            for ii in 0..2 {
                for jj in 0..2 {
                    a[i + ii][j + jj] += diff;
                }
            }
            ans += diff.abs();
        }
    }

    if a == b {
        println!("Yes");
        println!("{}", ans);
    } else {
        println!("No");
    }
}
