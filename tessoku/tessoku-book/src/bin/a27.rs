use proconio::input;

fn main() {
    input! {
      a: usize,
      b: usize,
    }

    println!("{}", gcd(a, b));
}

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        return b;
    } else if b == 0 {
        return a;
    }
    if a > b {
        return gcd(a % b, b);
    } else {
        return gcd(a, b % a);
    }
}
