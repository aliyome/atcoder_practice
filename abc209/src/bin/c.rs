use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        mut c: [usize; n]
    };

    c.sort();

    let mut ans = 1;
    for i in 0..n {
        ans *= c[i] - i;
        ans %= MOD;
    }

    println!("{}", ans);
}
