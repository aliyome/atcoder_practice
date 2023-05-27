use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[Usize1; n]; m],
    }

    let mut good = vec![vec![false; n]; n];
    for i in 0..m {
        for j in 0..n - 1 {
            good[a[i][j]][a[i][j + 1]] = true;
            good[a[i][j + 1]][a[i][j]] = true;
        }
    }

    let mut count = 0;
    for i in 0..n {
        for j in i + 1..n {
            if !good[i][j] {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
