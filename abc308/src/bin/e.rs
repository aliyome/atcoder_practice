use proconio::input;
use proconio::marker::Chars;

fn mex(bit: usize) -> usize {
    if bit == 0 {
        return 0;
    }

    for i in 0..10 {
        if bit & 1 << i == 0 {
            return i;
        }
    }
    panic!();
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        s: Chars,
    }

    // dp[x][y][z] := Sxまでみて、MEXのうちy文字目まで完成しているとき、すでに使われた数字をビット列zで表したときの場合の数
    let mut dp = vec![vec![vec![0; 1 << 3]; 4]; n + 1];
    // 0文字目までみてまだ1文字も完成していないとき、すでに使われた数字はないという状態
    dp[0][0][0] = 1;

    // i 文字目までみて
    for x in 0..n {
        // すでに j 文字完成していて
        for y in 0..=3 {
            // すでに使われた数字をビット列zで表したとき
            for z in 0..1 << 3 {
                // i + 1 文字目は現時点の場合の数を引き継ぎつつ
                dp[x + 1][y][z] += dp[x][y][z];
                // i + 1 文字目が j + 1 文字目になる場合
                if (y == 0 && s[x] == 'M') || (y == 1 && s[x] == 'E') || (y == 2 && s[x] == 'X') {
                    // すでに使われた数字をビット列zで表したときの場合の数に足す
                    dp[x + 1][y + 1][z | 1 << a[x]] += dp[x][y][z];
                }
            }
        }
    }

    let mut ans = 0;
    for z in 0..8 {
        ans += mex(z) * dp[n][3][z];
    }
    println!("{}", ans);
}
