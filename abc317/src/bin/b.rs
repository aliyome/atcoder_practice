use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    // 最小値と最大値を見つける
    let mut min_val = 1001;
    let mut max_val = 0;
    let mut sum = 0;
    for &x in &a {
        if x < min_val {
            min_val = x;
        }
        if x > max_val {
            max_val = x;
        }
        sum += x;
    }

    // N+1 個の整数の和を計算する
    let total_sum = (min_val + max_val) * (n as i32 + 1) / 2;

    // 失われた整数を見つける
    let lost_number = total_sum - sum;

    // 結果を出力する
    println!("{}", lost_number);
}
