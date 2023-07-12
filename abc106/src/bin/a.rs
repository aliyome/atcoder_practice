use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };

    println!("{}", a * b - a - b + 1);
}
