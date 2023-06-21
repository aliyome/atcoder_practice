use proconio::input;

fn main() {
    input! {
      n: usize,
      k: usize,
      a: [usize; k]
    }

    let mut dp = vec![false; n + 1];
    // 手番 i が
    for i in 0..=n {
        let mut win = false;
        for &a in &a {
            // 手番 i で a 個取ったときに相手を負けにできるなら、勝ち
            if a <= i && !dp[i - a] {
                win = true;
                break;
            }
        }
        dp[i] = win;
    }

    if dp[n] {
        println!("First");
    } else {
        println!("Second");
    }
}
