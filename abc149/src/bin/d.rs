use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        r: usize,
        s: usize,
        p: usize,
        t: Chars
    };

    let conv = |c: char| -> usize {
        if c == 'r' {
            0
        } else if c == 's' {
            1
        } else {
            2
        }
    };

    let win = |c: usize| -> usize {
        if c == 0 {
            2
        } else if c == 1 {
            0
        } else {
            1
        }
    };

    let win_score = |c: usize| -> usize {
        if c == 0 {
            p
        } else if c == 1 {
            r
        } else {
            s
        }
    };

    // [
    //   [1, k, 2k,...]
    //   [2, k+1, 2k+1,...]
    //   [3, k+2, 2k+2,...] のように分ける
    // ]
    let mut list = vec![];
    for i in 0..k {
        let mut tmp = vec![];
        for j in (i..n).step_by(k) {
            tmp.push(conv(t[j]));
        }
        list.push(tmp);
    }
    // println!("{:?}", list);

    let mut ans = 0;
    for list in &list {
        // dp[i][j] := i番目まで見た時に、最後にjを選んだときの最大値
        let mut dp = vec![vec![0; 3]; list.len() + 1];
        dp[0][win(list[0])] = win_score(list[0]);

        for i in 1..list.len() {
            // c 相手の手
            let c = list[i];
            // 今回勝つ手
            let w = win(c);
            // 今回買った場合のスコア
            let ws = win_score(c);
            // j 今回の手
            for j in 0..3 {
                // k 前回の手
                for k in 0..3 {
                    if j == w {
                        // 勝ち
                        if j == k {
                            // だけど前回と同じ手
                            // dp[i][j] = dp[i][j].max(dp[i - 1][j] + ws);
                        } else {
                            dp[i][j] = dp[i][j].max(dp[i - 1][k] + ws);
                        }
                    } else {
                        // 勝たない
                        dp[i][j] = dp[i][j].max(dp[i - 1][k]);
                    }
                }
            }
        }
        ans += dp[list.len() - 1].iter().max().unwrap();
    }

    println!("{}", ans);
}
