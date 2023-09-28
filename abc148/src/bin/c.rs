use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };

    println!("{}", lcm(a, b));
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}
