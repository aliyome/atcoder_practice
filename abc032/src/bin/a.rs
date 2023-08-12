use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        n: usize,
    };
    let l = lcm(a, b);
    for i in 1.. {
        if l * i >= n {
            println!("{}", l * i);
            return;
        }
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
    a * b / gcd(a, b)
}
