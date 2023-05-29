use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        w: usize,
        n: usize,
        mut lrv: [(i64, i64, i64); n],
    }

    // l,rでソート
    let mut dishes = vec![];
    dishes.push((0, 0, 0)); // 1-indexedにするために0を追加
    dishes.extend(lrv);

    // dp[i][j] = i番目までの料理を使ってj[g]の香辛料を使うときの価値の最大値
    // dp[i][j] は以下の max
    // 1. i番目の料理を使わない場合 -> dp[i-1][j]
    // 2. i番目の料理を使う場合 -> dp[i-1][j - k] + v (k = [l,r])
    // 計算量 O(NW^2)
    //
    // 2. をスライド最大値を使って高速化する
    // 計算量 O(NW)

    let mut dp = vec![vec![std::i64::MIN; w + 1]; n + 1];
    dp[0][0] = 0;

    for i in 1..=n {
        let (l, r, v) = dishes[i];

        // スライド最大値
        let mut slide_max = VecDeque::new();
        let mut current_index = 0;
        for j in 0..=w {
            // i番目の料理を使わない場合はi-1番目までと同じ
            dp[i][j] = dp[i][j].max(dp[i - 1][j]);

            // i番目の料理を使う場合
            // スライド最大値が単純降下になるように末尾を調整する
            // dp[i][j - r] から dp[i][j - l] までのスライド最大値を求める

            // ..dp[i][j - r] はスライド範囲外なので削除
            let lower = j as i64 - r;
            while let Some(&(_, index)) = slide_max.front() {
                if index < lower {
                    slide_max.pop_front();
                } else {
                    break;
                }
            }

            // dp[i][j - r]..=dp[i][j - l] はスライド範囲内なので追加
            let upper = j as i64 - l;
            if current_index <= upper {
                while let Some(&(val, _)) = slide_max.back() {
                    // 末尾の値より大きい値が来たら末尾を削除
                    if val <= dp[i - 1][current_index as usize] {
                        slide_max.pop_back();
                    } else {
                        break;
                    }
                }
                slide_max.push_back((dp[i - 1][current_index as usize], current_index));
                current_index += 1;
            }

            // 最大値を使ってdpを更新
            if let Some(&(val, _)) = slide_max.front() {
                dp[i][j] = dp[i][j].max(val + v);
            }

            // // i番目の料理を使う場合 (愚直)
            // for k in l..=r {
            //     if j as i64 - k < 0 {
            //         continue;
            //     }
            //     let k = k as usize;
            //     dp[i][j] = dp[i][j].max(dp[i - 1][j - k] + v);
            // }
        }
    }

    println!("{}", dp[n][w].max(-1));
}
