use proconio::input;

fn main() {
    input! {
        n: usize,
        m: i64,
    }

    // ギフトの座標を入力し、ソート
    input! {
        mut a: [i64; n],
    }
    a.sort();

    // しゃくとり法で最大のギフト数を求める
    let mut max_gifts = 0;
    let mut left = 0;
    for right in 0..n {
        while a[right] - a[left] >= m {
            left += 1;
        }
        max_gifts = max_gifts.max(right - left + 1);
    }

    // 結果を出力
    println!("{}", max_gifts);
}
