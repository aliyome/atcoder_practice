use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        k: usize
    }

    // (N mod 9) = (N の各桁の和 mod 9)
    if k % 9 != 0 {
        println!("0");
        return;
    }

    // dp[s] = 各桁の和が s になるような数の個数
    let mut dp = vec![0; k + 1];
    // 総和が 0 になるような数は 1 つだけ = 何も選ばない
    dp[0] = 1;
    for s in 1..=k {
        // 総和が s - d になるような数の末尾に1桁 d を追加することで総和が s になるような数を作ることができる
        // 例えば、総和が 5 になるような数は、総和が 4,3,2,1,0 になるような数の末尾にそれぞれ 1,2,3,4,5 を追加することで作ることができる
        for d in 1..=9 {
            if s as isize - d as isize >= 0 {
                dp[s] += dp[s - d];
                dp[s] %= MOD;
            }
        }
    }

    println!("{:?}", dp[k]);
}
