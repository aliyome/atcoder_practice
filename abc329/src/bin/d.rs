use proconio::input;

fn main() {
    input! {
        n: usize, // 候補者の数
        m: usize, // 投票の数
        votes: [usize; m], // 各投票
    }

    let mut counts = vec![0; n + 1]; // 各候補者の得票数を格納する配列
    let mut current_max = 0; // 現在の最大得票数
    let mut current_winner = 0; // 現在の勝者

    for i in 0..m {
        let vote = votes[i];
        counts[vote] += 1; // 対応する候補者の得票数を増やす

        // 新しい得票数が現在の最大得票数以上の場合、勝者を更新
        if counts[vote] > current_max {
            current_max = counts[vote];
            current_winner = vote;
        } else if counts[vote] == current_max {
            // 最大得票数が同数の場合、番号が小さい候補者を選ぶ
            if vote < current_winner {
                current_winner = vote;
            }
        }

        println!("{}", current_winner);
    }
}
