use proconio::input;

fn main() {
    input! {
        x: i128,
    };

    if x < 0 && x % 10 != 0 {
        println!("{}", x / 10 - 1);
    } else {
        println!("{}", x / 10);
    }
}
