use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    // 末尾の0の個数を計算する
    // `trailing_zeros()`メソッドを使って、2進数表記の末尾の0の数を求める
    let ctz = n.trailing_zeros();

    // 結果を出力する
    println!("{}", ctz);
}
