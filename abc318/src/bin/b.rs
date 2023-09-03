use proconio::input;

fn main() {
    input! {
        n: usize,
        abcd: [(usize, usize, usize, usize); n]
    }

    let mut imos = vec![vec![0; 102]; 102];
    for &(a, b, c, d) in &abcd {
        imos[a][c] += 1;
        imos[b][c] -= 1;
        imos[b][d] += 1;
        imos[a][d] -= 1;
    }
    for i in 0..=100 {
        for j in 0..=100 {
            imos[i][j + 1] += imos[i][j];
        }
    }
    for j in 0..=100 {
        for i in 0..=100 {
            imos[i + 1][j] += imos[i][j];
        }
    }

    let mut ans = 0;
    for i in 0..=100 {
        for j in 0..=100 {
            if imos[i][j] > 0 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
