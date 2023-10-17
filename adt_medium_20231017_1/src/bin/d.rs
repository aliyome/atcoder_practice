// adt_medium_20231017_1
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    // すべての人が他の人と一緒に同じパーティーに参加したかどうかをチェックするための配列を初期化する
    let mut ok = vec![vec![false; n + 1]; n + 1];

    // 各パーティーでの参加者を確認し、彼らが会ったことをマークする
    for _ in 0..m {
        input! {
            k: usize,
            mut x: [usize; k],
        };

        for i in 0..x.len() {
            for j in i + 1..x.len() {
                ok[x[i]][x[j]] = true;
                ok[x[j]][x[i]] = true;
            }
        }
    }

    // すべてのペアが一緒にパーティーに参加したことがあるかどうかを確認する
    for i in 1..=n {
        for j in i + 1..=n {
            if !ok[i][j] {
                // 一緒にパーティーに参加していないペアが見つかった場合
                println!("No");
                return;
            }
        }
    }

    // すべてのペアが一緒に少なくとも1回のパーティーに参加した
    println!("Yes");
}
