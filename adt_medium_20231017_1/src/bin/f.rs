use proconio::input;

fn main() {
    input! {
        _n: usize,
        s: String,
    }

    // 現在の位置を (x, y) とする
    let mut x = 0;
    let mut y = 0;

    // すでに訪れた座標を保持するためのHashSetを使用する
    use std::collections::HashSet;
    let mut visited = HashSet::new();

    // 初期位置を訪れた座標として登録
    visited.insert((x, y));

    for ch in s.chars() {
        match ch {
            'R' => x += 1,
            'L' => x -= 1,
            'U' => y += 1,
            'D' => y -= 1,
            _ => {}
        }

        // すでにその座標を訪れていれば "Yes" を出力して終了
        if visited.contains(&(x, y)) {
            println!("Yes");
            return;
        }
        visited.insert((x, y));
    }

    // すべての移動が終わっても同じ座標に戻らない場合 "No" を出力
    println!("No");
}
