use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    // dp[c][i] = i文字目以降で文字cが最初に出現する位置
    // 例(sがabaの場合):
    // dp[a][0] = 0
    // dp[a][1] = 2
    // dp[a][2] = 2
    // dp[b][0] = 1
    // dp[b][1] = 1
    // dp[b][2] = -1  // 出現しない
    let mut dp = vec![vec![std::usize::MAX; n + 1]; 26];

    // 末尾からdpを埋めていく
    for c in 0..26 {
        for i in (0..n).rev() {
            let letter = (c as u8 + b'a') as char;
            if s[i] == letter {
                dp[c][i] = i;
            } else {
                dp[c][i] = dp[c][i + 1];
            }
        }
    }

    // 貪欲法
    let mut idx: usize = 0;
    let mut l = 0;
    for _ in 0..k {
        for c in 0..26 {
            // idx以降(idxを含む)の最小の出現位置
            let next_idx = dp[c][idx];
            // 出現しない文字はスキップ
            if next_idx == std::usize::MAX {
                continue;
            }
            // 添字が N - K + l より大きい場合はK文字作れないのでスキップ
            if next_idx > n - k + l {
                continue;
            }
            print!("{}", (c as u8 + b'a') as char);
            l += 1;
            // 参照位置を更新
            idx = next_idx + 1;
            break;
        }
    }

    println!("");
}
