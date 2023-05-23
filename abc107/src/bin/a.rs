use proconio::input;

fn main() {
    input! {
        n: usize,
        i: usize,
    };

    println!("{}", n - i + 1);
}
