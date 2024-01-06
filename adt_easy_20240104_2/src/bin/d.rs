use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    };

    let mut b = a.clone();
    // 0 行目
    for j in 0..n - 1 {
        b[0][j + 1] = a[0][j];
    }
    // n 列目
    for i in 0..n - 1 {
        b[i + 1][n - 1] = a[i][n - 1];
    }
    // n 行目
    for j in (1..n).rev() {
        b[n - 1][j - 1] = a[n - 1][j];
    }
    // 0 列目
    for i in (1..n).rev() {
        b[i - 1][0] = a[i][0];
    }

    for i in 0..n {
        for j in 0..n {
            print!("{}", b[i][j]);
        }
        println!("");
    }
}
