use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }

    let r = b / gcd(a, b);

    if r > 10usize.pow(18) / a {
        println!("Large");
    } else {
        println!("{}", r * a);
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
