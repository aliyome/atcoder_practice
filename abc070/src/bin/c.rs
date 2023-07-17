use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [u128; n]
    };

    let mut ans = t[0];
    for i in 1..n {
        ans = lcm(ans, t[i]);
    }

    println!("{}", ans);
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u128, b: u128) -> u128 {
    a * b / gcd(a, b)
}
