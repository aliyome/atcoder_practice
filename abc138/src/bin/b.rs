use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    let mut x = 1;
    for i in 0..n {
        x = lcm(x, a[i]);
    }

    let mut sum = 0f64;
    for i in 0..n {
        sum += x as f64 / a[i] as f64;
    }
    println!("{}", x as f64 / sum);
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
