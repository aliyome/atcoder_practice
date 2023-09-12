use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    };

    if x <= a {
        println!("{}", 1.0f64);
    } else if x <= b {
        println!("{}", c as f64 / (b - a) as f64);
    } else {
        println!("{}", 0.0f64);
    }
}
