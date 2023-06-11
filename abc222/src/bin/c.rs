use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Chars; n * 2]
    };

    // orders[i] := (i, 勝利数)
    let mut players = vec![];
    players.push((0, 100000000));
    for i in 1..=n * 2 {
        players.push((i, 0));
    }

    // O(M)各ラウンドの対戦結果を計算
    for i in 0..m {
        // println!("{:?}", players);
        // O(N) 各対戦結果を計算
        // 順位順に 1vs2, 3vs4, ... と対戦
        for j in (1..n * 2).step_by(2) {
            // players[j] と players[j + 1] が対戦
            let p1 = players[j];
            let p2 = players[j + 1];

            // a[2n][m] := n人目の人のm回目の手 なので
            // p1 の手は a[p1][i] で表される
            let lhs = a[p1.0 - 1][i];
            let rhs = a[p2.0 - 1][i];

            // 対戦結果を計算
            match fight(lhs, rhs) {
                0 => {
                    players[j].1 += 1;
                }
                1 => {}
                2 => {
                    players[j + 1].1 += 1;
                }
                _ => panic!("unexpected result"),
            };
            // println!(
            //     "{}:{} vs {}:{} => {}",
            //     p1.0,
            //     lhs,
            //     p2.0,
            //     rhs,
            //     fight(lhs, rhs)
            // );
        }
        // O(NlogN) 勝利数順, プレイヤー番号順にソート
        players.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    }

    // println!("{:?}", players);

    // 出力
    for i in 1..=n * 2 {
        println!("{}", players[i].0);
    }
}

// 0: lhs win, 1: draw, 2: rhs win
fn fight(lhs: char, rhs: char) -> usize {
    if lhs == 'G' && rhs == 'C' {
        0
    } else if lhs == 'C' && rhs == 'P' {
        0
    } else if lhs == 'P' && rhs == 'G' {
        0
    } else if lhs == rhs {
        1
    } else {
        2
    }
}
