use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        t: usize,
    };

    if n % x == 0 {
        println!("{}", n / x * t);
    } else {
        println!("{}", (n / x + 1) * t);
    }
}
