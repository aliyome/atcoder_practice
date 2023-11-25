use proconio::input;
use std::cmp;

fn main() {
    input! {
        d: i64,
    }

    let mut result = d;
    let mut x = 0;

    while x * x <= d {
        let y = ((d - x * x) as f64).sqrt() as i64;
        // |x^2 + y^2 - D| の値を計算し、最小値を更新
        result = cmp::min(result, (x * x + y * y - d).abs());
        // y+1 の場合もチェック
        result = cmp::min(result, (x * x + (y + 1) * (y + 1) - d).abs());
        x += 1;
    }

    println!("{}", result);
}
