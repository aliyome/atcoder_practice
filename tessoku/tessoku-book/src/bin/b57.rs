use proconio::input;

fn main() {
    input! {
      n: usize, // <= 3 x 10^5
      k: usize, // <= 10^9
    }

    // ダブリング
    // dp[d][i] := i から 2^d 回移動したときの頂点番号 (10^9 < 2^30)
    let mut dp = vec![vec![0; n + 1]; 31];

    // 初期値 各桁の和を計算
    for i in 0..=n {
        dp[0][i] = (i as isize - digit_sum(i) as isize) as usize;
    }

    // 2^d 回移動したときの頂点番号を計算
    for d in 1..=30 {
        for i in 0..=n {
            dp[d][i] = dp[d - 1][dp[d - 1][i]];
        }
    }

    // 各整数i に対して
    for i in 1..=n {
        // k 回移動したときの頂点番号を計算
        let mut i = i;
        let mut remainder = k;
        while remainder > 0 {
            let d = (remainder as f64).log2().floor() as usize;
            remainder -= 2usize.pow(d as u32);
            i = dp[d][i];
        }
        println!("{}", i);
    }
}

fn digit_sum(mut n: usize) -> usize {
    let mut sum = 0;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}
