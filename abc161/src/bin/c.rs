use proconio::input;

fn main() {
    input! {
        n: isize,
        k: isize
    };

    let d = n / k;
    let a = (n - k * d).abs();
    let b = (n - k * (d + 1)).abs();
    println!("{}", a.min(b));
}
