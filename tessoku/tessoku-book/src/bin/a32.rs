use proconio::input;

fn main() {
    input! {
      n: usize,
      a: usize,
      b: usize
    }

    // 石が x になった時、その手番の人が負け
    let x = a.min(b) - 1;
    // dp[i] := 石が i になった時、その手番の人が勝つかどうか
    let mut dp = vec![false; n + 1];
    for i in 0..=x {
        dp[i] = false;
    }

    for i in x + 1..=n {
        let a_win = a <= i && dp[i - a];
        let b_win = b <= i && dp[i - b];
        dp[i] = !(a_win && b_win);
        // println!(
        //     "i: {}, a_win: {}, b_win: {}, win?: {}",
        //     i, a_win, b_win, dp[i]
        // );
    }

    if dp[n] {
        println!("First");
    } else {
        println!("Second");
    }
}
