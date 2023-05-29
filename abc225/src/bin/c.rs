use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        b: [[Usize1; m]; n]
    };

    for i in 0..n {
        for j in 0..m {
            if b[i][j] != b[0][0] + (i * 7) + j || b[0][0] / 7 != b[0][m - 1] / 7 {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
