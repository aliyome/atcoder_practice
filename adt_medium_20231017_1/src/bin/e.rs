use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u32; n],
        b: [u32; m]
    }

    // A と B の要素を一つずつ順番に確認していき、小さい方を C に追加していく
    let mut ai = 0;
    let mut bi = 0;
    let mut a_positions = Vec::new();
    let mut b_positions = Vec::new();

    while ai < n || bi < m {
        if bi >= m || (ai < n && a[ai] < b[bi]) {
            a_positions.push(ai + bi + 1); // 1-indexed なので +1
            ai += 1;
        } else {
            b_positions.push(ai + bi + 1); // 1-indexed なので +1
            bi += 1;
        }
    }

    // 結果を出力
    // a_positions をスペース区切りの文字列に変換する
    println!(
        "{}",
        a_positions
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
    println!(
        "{}",
        b_positions
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
