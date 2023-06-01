use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n]
    };

    let mut g = a[0];
    for i in 1..n {
        g = gcd(g, a[i]);
    }

    println!("{}", g);
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
