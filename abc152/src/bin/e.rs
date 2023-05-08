use proconio::input;

const MOD: i128 = 1_000_000_007;

fn gcd(a: i128, b: i128) -> i128 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcm(a: i128, b: i128) -> i128 {
    a * b / gcd(a, b)
}

fn main() {
    input! {
        n: usize,
        a: [i128; n],
    }

    let mut l = a[0];
    for i in 1..n {
        l = lcm(l, a[i]);
    }

    let mut ans = 0;
    for i in 0..n {
        ans += l / a[i];
    }
    println!("{}", ans % MOD);
}
