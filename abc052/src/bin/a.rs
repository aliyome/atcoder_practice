use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    };

    println!("{}", (a * b).max(c * d));
}
