use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    // 各文字の出現位置を記録
    let mut pos = vec![vec![false; 10usize.pow(5) + 1]; 26];
    for i in 0..n {
        let c = s[i] as usize - 'a' as usize;
        pos[c][i] = true;
    }

    // 全探索
    // k文字作れるまで
    let mut idx = 0;
    let mut ans = vec![];
    for j in 0..k {
        // a から順番に
        'outer: for c in 0..26 {
            // その文字の出現位置を順番に見ていく
            for i in idx..n {
                if !pos[c][i] {
                    continue;
                }
                // 出現位置が後ろすぎる場合はスキップ
                if i + k - j > n {
                    continue;
                }
                idx = i + 1;
                ans.push((c as u8 + 'a' as u8) as char);
                break 'outer;
            }
        }
    }

    println!("{}", ans.iter().collect::<String>());
}
