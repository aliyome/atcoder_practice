use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    // 合計値を計算
    let sum: i64 = a.iter().sum();

    // 平均値（整数部分）を計算
    let avg = sum / n as i64;

    // 操作回数を初期化
    let mut ops = 0;

    // 各要素に対して、平均値との差の絶対値を計算し、操作回数に追加
    for &x in &a {
        ops += (x - avg).abs();
    }
    println!("{}", ops / 2);
}
