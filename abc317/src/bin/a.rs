use proconio::input;

fn main() {
    input! {
        n: usize,
        h: i32,
        x: i32,
        potions: [i32; n]
    }

    let target = x - h;

    // バイナリサーチで目標値以上のポーションを探す
    let mut low = 0;
    let mut high = n;
    while low < high {
        let mid = (low + high) / 2;
        if potions[mid] < target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    // 1-based indexなので+1して出力
    println!("{}", low + 1);
}
