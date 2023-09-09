use proconio::input;

fn check_width(width: i64, word_lengths: &Vec<i64>, m: usize) -> bool {
    let mut lines = 1;
    let mut current_width = 0;

    for &length in word_lengths {
        if length > width {
            return false; // 単語がウィンドウ幅を超える場合
        }
        if current_width + length + if current_width > 0 { 1 } else { 0 } > width {
            lines += 1;
            current_width = 0;
        }

        current_width += length + if current_width > 0 { 1 } else { 0 };
    }

    lines <= m
}

fn main() {
    input! {
        n: usize,
        m: usize,
        l: [i64; n]
    }

    let mut left = *l.iter().max().unwrap(); // 最大の単語の長さ
    let mut right = l.iter().sum::<i64>() + (n as i64) - 1; // すべての単語とスペースの合計

    while left < right {
        let mid = (left + right) / 2;
        if check_width(mid, &l, m) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    println!("{}", left);
}
