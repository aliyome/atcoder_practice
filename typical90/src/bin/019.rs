use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n * 2]
    }

    // 隣り合った値の差が小さいものから取り除く　→　Wrong Answer :sob:
    let mut ans = 0;
    let mut del_idx = vec![];
    for _ in 0..n {
        // 比較対象のインデックス
        let mut from = std::usize::MAX;
        let mut to = std::usize::MAX;
        // 削除対象
        let mut del_ids = (std::usize::MAX, std::usize::MAX);
        // diffの最小値
        let mut min = 10i64.pow(6) + 5;
        // 前から順番に取り除かれていない要素を比較していく
        for j in 0..n * 2 {
            if del_idx.contains(&j) {
                continue;
            }
            if from == std::usize::MAX {
                from = j;
                continue;
            }
            to = j;

            // diffを計算
            let diff = (a[from] - a[to]).abs();
            // 削除対象の更新
            if min > diff {
                min = diff;
                del_ids = (from, to);
            }

            // 次のループ用にfromを更新
            from = to;
        }

        // 削除対象のインデックスを記録
        del_idx.push(del_ids.0);
        del_idx.push(del_ids.1);

        // 答えに加算
        ans += min;
        // println!("{} - {} = {}", a[del_ids.0], a[del_ids.1], min);
    }

    println!("{}", ans);
}
