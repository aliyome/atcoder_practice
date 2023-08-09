use proconio::{input, marker::Chars};

fn main() {
    input! {
        k: usize,
        mut s: Chars,
        mut t: Chars,
    };

    s.pop();
    t.pop();

    // カードの残数
    let mut cards = vec![k; 10];
    cards[0] = 0;
    for i in 0..4 {
        cards[s[i].to_digit(10).unwrap() as usize] -= 1;
        cards[t[i].to_digit(10).unwrap() as usize] -= 1;
    }

    // スコアの計算
    let calc_score = |cards: &Vec<char>| {
        let mut digits = vec![0; 10];
        for &c in cards {
            digits[c.to_digit(10).unwrap() as usize] += 1;
        }

        let mut score = 0;
        for i in 1..10 {
            score += i * 10_usize.pow(digits[i] as u32);
        }

        score
    };

    // 高橋くん青木くんの最後の手がそれぞれi,jだった時の場合の数を計算
    let mut win = 0;
    for i in 1..10 {
        for j in 1..10 {
            // カードが足りない場合は発生しない事象なのでスキップ
            if i == j && cards[i] < 2 {
                continue;
            }
            if cards[i] < 1 || cards[j] < 1 {
                continue;
            }

            // 最終スコア
            let mut ss = s.clone();
            ss.push(i.to_string().chars().next().unwrap());
            let mut tt = t.clone();
            tt.push(j.to_string().chars().next().unwrap());

            let s_score = calc_score(&ss);
            let t_score = calc_score(&tt);

            // 高橋くんが勝つ場合の数
            if s_score > t_score {
                if i == j {
                    win += cards[i] * (cards[i] - 1);
                } else {
                    win += cards[i] * cards[j];
                }
            }
        }
    }

    let rest_cards = cards.iter().sum::<usize>();
    let total = rest_cards * (rest_cards - 1);
    println!("{}", win as f64 / total as f64);
}
