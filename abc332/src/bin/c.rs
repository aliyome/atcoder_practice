use proconio::input;

fn main() {
    // 入力の受け取り
    input! {
        n: usize,
        m: usize,
        s: String,
    }

    let mut logo = 0;
    let mut muji = m;
    let mut used_logo = 0;
    let mut used_muji = 0;

    // 各日についての処理
    for c in s.chars() {
        match c {
            '1' => {
                // 外食の日：無地かロゴのTシャツを着る
                if muji > 0 {
                    muji -= 1;
                    used_muji += 1;
                } else if logo > 0 {
                    logo -= 1;
                    used_logo += 1;
                } else {
                    // どちらもない場合は、ロゴのTシャツを買う
                    used_logo += 1;
                }
            }
            '2' => {
                // プログラミングイベントの日：ロゴのTシャツを着る
                if logo > 0 {
                    logo -= 1;
                    used_logo += 1;
                } else {
                    // ロゴのTシャツがない場合は、ロゴのTシャツを買う
                    used_logo += 1;
                }
            }
            '0' => {
                // 予定なしの日：Tシャツを洗濯
                muji += used_muji;
                logo += used_logo;
                used_muji = 0;
                used_logo = 0;
            }
            _ => unreachable!(),
        }
    }

    // 必要なTシャツの最小数を出力
    println!("{}", logo + used_logo);
}
