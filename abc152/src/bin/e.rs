use proconio::input;

const MOD: i64 = 1_000_000_007;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut lcm = a[0];
    for i in 1..n {
        lcm = lcm * a[i] / gcd(lcm, a[i]);
    }

    let mut ans = 0;
    for i in 0..n {
        ans = (ans + lcm / a[i]) % MOD;
    }

    println!("{}", ans);
}
