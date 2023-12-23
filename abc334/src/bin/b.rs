use proconio::input;

fn main() {
    // 入力
    input! {
        a: i64,
        m: i64,
        l: i64,
        r: i64,
    }

    // L, Rの間でA+kMの形で表される点の数を計算する
    let count = count_trees(a, m, l, r);

    // 出力
    println!("{}", count);
}

// クリスマスツリーの数を計算する関数
fn count_trees(a: i64, m: i64, l: i64, r: i64) -> i64 {
    // LとRの間で最初と最後のツリーの位置を求める
    let first_tree = if (l - a).rem_euclid(m) == 0 {
        l
    } else {
        l + (m - (l - a).rem_euclid(m))
    };
    let last_tree = r - (r - a).rem_euclid(m);

    // ツリーが一つもない場合
    if first_tree > last_tree {
        return 0;
    }

    // ツリーの総数を計算
    ((last_tree - first_tree) / m) + 1
}
