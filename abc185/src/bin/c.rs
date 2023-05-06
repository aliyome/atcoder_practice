use proconio::input;

fn main() {
    input! {
        l: i64,
    }
    let mut result: i64 = 1;
    for i in 1..12 {
        result = result * (l - i) / i;
    }
    println!("{}", result);
}
