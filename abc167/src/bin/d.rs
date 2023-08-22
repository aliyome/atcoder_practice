use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n]
    };

    // 1-indexed
    a.insert(0, 0);

    // dp[i][j] := j から 2^i 回移動した頂点番号
    let mut dp = vec![vec![0; n + 1]; 61];
    for j in 0..=n {
        dp[0][j] = a[j];
    }

    for i in 1..=60 {
        for j in 0..=n {
            dp[i][j] = dp[i - 1][dp[i - 1][j]];
        }
    }

    // for i in 0..3 {
    //     println!("{:?}", dp[i]);
    // }

    // k 回移動したときの頂点番号を計算
    let mut i = 1;
    let mut remainder = k;
    while remainder > 0 {
        let d = (remainder as f64).log2().floor() as usize;
        remainder -= 2usize.pow(d as u32);
        i = dp[d][i];
    }
    println!("{}", i);
}
