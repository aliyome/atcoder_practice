use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, // 1 <= n <= 3 x 10^5
        a: [i64; n] // -200 <= a_i <= 200
    };

    // a を自然数に変換
    let a = a.iter().map(|x| (x + 200) as usize).collect_vec();

    // cnt[i][j] := i 番目までに含まれる j の数
    let mut cnt = vec![vec![0; 401]; n + 1];
    cnt[0][a[0]] = 1;
    for i in 1..n {
        cnt[i] = cnt[i - 1].clone();
        cnt[i][a[i]] += 1;
    }

    let mut ans = 0;

    // Σ_{i=1}^{n} Σ_{j=0}^{i-1} (a[i] - a[j])^2
    for i in 1..n {
        for j in 0..=400 {
            ans += (a[i] - j).pow(2) * cnt[i - 1][j];
        }
    }

    println!("{}", ans);
}
