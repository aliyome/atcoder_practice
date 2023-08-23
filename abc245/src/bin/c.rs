use proconio::input;

fn main() {
    input! {
        n: usize,
        k: isize,
        a: [isize; n],
        b: [isize; n]
    };

    // dp[i][j] := i番目まで見た時に j が成立するか (j = 0:a, 1:b)
    let mut dp = vec![vec![false; 2]; n];
    dp[0][0] = true;
    dp[0][1] = true;

    for i in 1..n {
        if dp[i - 1][0] {
            if (a[i] - a[i - 1]).abs() <= k {
                dp[i][0] |= true;
            }
            if (b[i] - a[i - 1]).abs() <= k {
                dp[i][1] |= true;
            }
        }
        if dp[i - 1][1] {
            if (a[i] - b[i - 1]).abs() <= k {
                dp[i][0] |= true;
            }
            if (b[i] - b[i - 1]).abs() <= k {
                dp[i][1] |= true;
            }
        }
    }

    if dp[n - 1][0] || dp[n - 1][1] {
        println!("Yes");
    } else {
        println!("No");
    }
}
