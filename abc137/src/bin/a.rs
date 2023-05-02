use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64
    };

    println!("{}", (a + b).max(a - b).max(a * b));
}
