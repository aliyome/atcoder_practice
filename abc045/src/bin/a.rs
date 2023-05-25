use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        h: usize,
    };

    let n = (a + b) * h / 2;

    println!("{}", n);
}
