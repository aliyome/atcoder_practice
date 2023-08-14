use proconio::input;

const MOD: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        a: [[usize; 6]; n],
    };

    let mut sums = vec![0; n];
    for i in 0..n {
        for j in 0..6 {
            sums[i] += a[i][j];
            sums[i] %= MOD;
        }
    }

    let mut ans = 1;
    for i in 0..n {
        ans *= sums[i];
        ans %= MOD;
    }

    println!("{}", ans);
}
