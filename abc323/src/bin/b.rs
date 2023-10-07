use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    // 各プレイヤーの勝利数をカウントする
    let mut win_counts = vec![];
    for i in 0..n {
        let count = s[i].chars().filter(|&c| c == 'o').count();
        win_counts.push((count, -(i as isize))); // 勝利数と-index（indexは昇順でのソートを保証するため）
    }

    // 勝利数で降順にソート
    win_counts.sort_by(|a, b| b.cmp(a));

    // ソート後のプレイヤーの順番を表示
    for (_, index) in win_counts {
        print!("{} ", (-index) + 1); // indexを元の正の値に戻して出力
    }
}
