use proconio::input;

fn main() {
    input! {
      a: usize,
      b: usize
    }

    println!("{}", lcm(a, b));
}

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }

    if a < b {
        gcd(a, b % a)
    } else {
        gcd(a % b, b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}
