use proconio::input;

fn main() {
    input! {
        x: usize,
    };

    let mut dp = vec![false; 100001];
    dp[0] = true;

    for i in 0..100000 {
        if !dp[i] {
            continue;
        }
        for &v in &[100, 101, 102, 103, 104, 105] {
            if i + v > 100000 {
                continue;
            }
            dp[i + v] = true;
        }
    }

    if dp[x] {
        println!("1");
    } else {
        println!("0");
    }
}
