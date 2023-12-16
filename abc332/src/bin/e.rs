use proconio::input;

fn main() {
    input! {
        n: usize, // アイテムの数
        d: usize, // ラッキーバッグの数
        weights: [i64; n], // 各アイテムの重さ
    }

    // 動的計画法のテーブルを初期化（すべての値を最大値で初期化）
    // dp[i][b] := i に番目のアイテムを、ビット状態 b の袋に入れたときの重さの合計
    let mut dp: Vec<Vec<i64>> = vec![vec![i64::MAX; 1 << d]; n + 1];
    dp[0][0] = 0;

    // i に番目のアイテムを追加する
    for i in 0..n {
        // b はビット状態
        for b in 0..1 << d {
            if dp[i][b] != i64::MAX {
                // アイテムを追加しない場合
                dp[i + 1][b] = dp[i + 1][b].min(dp[i][b]);
                // アイテムを追加する場合
                let new_weight = dp[i][b] + weights[i];
                dp[i + 1][new_weight] = dp[i][b] + weights[i].pow(2);
            }
        }
    }

    // 最終的な答えを見つける
    let total_weight: i64 = weights.iter().sum();
    let mut answer = i64::MAX;
    for j in 0..=30000 {
        if dp[n][j] != i64::MAX {
            let x = total_weight - j as i64;
            let current_variance = dp[n][j] - 2 * j * x + d as i64 * x.pow(2);
            answer = answer.min(current_variance);
        }
    }

    // 答えを出力（分散を d で割る）
    println!("{}", answer as f64 / d as f64);
}
