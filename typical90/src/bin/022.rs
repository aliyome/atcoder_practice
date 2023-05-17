use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    }

    let x = gcd(gcd(a, b), c);
    let ans = a / x + b / x + c / x - 3;
    println!("{}", ans);
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
