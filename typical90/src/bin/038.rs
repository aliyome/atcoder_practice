use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }

    if b / gcd(a, b) > 10usize.pow(18) / a {
        println!("Large");
    } else {
        println!("{}", lcm(a, b));
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    b / gcd(a, b) * a
}
